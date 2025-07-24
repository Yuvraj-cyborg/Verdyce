//! # Proposal Model
//!
//! Defines the core proposal structure and evaluation logic for the consensus system.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::decay::DecayModel;
use crate::models::vote::{Vote, VoteChoice, calculate_vote_weight};
use crate::threshold::{ThresholdModel, threshold_calc};
use crate::window::VotingWindow;

/// Represents the current status of a proposal in the consensus system.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProposalStatus {
    /// Proposal is actively accepting votes
    Pending,
    /// Proposal has met the approval threshold and is accepted
    Accepted,
    /// Proposal failed to meet the threshold within the time limit
    Rejected,
    /// Proposal exceeded the grace period without resolution
    Expired,
}

/// A proposal in the consensus system with associated voting logic.
///
/// Each proposal contains all the information needed to manage its lifecycle:
/// - Basic metadata (title, description, creation time)
/// - Voting configuration (decay model, threshold model, voting window)
/// - Current state (votes, status)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    /// Unique identifier for this proposal
    pub id: Uuid,
    /// Human-readable title
    pub title: String,
    /// Detailed description of what this proposal is for
    pub description: String,
    /// When this proposal was created
    pub created_at: DateTime<Utc>,
    /// All votes cast on this proposal
    pub votes: Vec<Vote>,
    /// Current status of the proposal
    pub status: ProposalStatus,
    /// Voting window configuration and state
    pub voting_window: VotingWindow,
    /// Model for how vote weights decay over time
    pub decay_model: DecayModel,
    /// Model for how approval thresholds change over time
    pub threshold_model: ThresholdModel,
}

impl Proposal {
    /// Creates a new proposal with the specified parameters.
    ///
    /// # Arguments
    /// * `title` - Human-readable title for the proposal
    /// * `description` - Detailed description of the proposal
    /// * `duration` - Voting period duration in seconds
    /// * `decay_model` - How vote weights should decay over time
    /// * `threshold_model` - How approval thresholds should change over time
    pub fn new(
        title: String,
        description: String,
        duration: u64,
        decay_model: DecayModel,
        threshold_model: ThresholdModel,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            title,
            description,
            created_at: now,
            votes: Vec::new(),
            status: ProposalStatus::Pending,
            voting_window: VotingWindow::new(now, duration, 30),
            decay_model,
            threshold_model,
        }
    }

    /// Adds a vote to this proposal.
    ///
    /// # Arguments
    /// * `vote` - The vote to add
    pub fn add_vote(&mut self, vote: Vote) {
        self.votes.push(vote);
    }

    /// Evaluates the current state of the proposal and updates its status.
    ///
    /// This method checks:
    /// - If the proposal has expired (past grace period)
    /// - If the proposal has met the approval threshold (accepted)
    /// - If the voting period has ended without meeting threshold (rejected)
    ///
    /// # Arguments
    /// * `now` - Current timestamp for evaluation
    pub fn evaluate(&mut self, now: DateTime<Utc>) {
        if self.status != ProposalStatus::Pending {
            return;
        }

        let elapsed = self.voting_window.elapsed(now);
        let total = self.voting_window.total_duration();
        let grace_cutoff = total + self.voting_window.grace_period;

        if elapsed >= grace_cutoff {
            self.status = ProposalStatus::Expired;
            return;
        }

        let threshold = threshold_calc(&self.threshold_model, elapsed, total);
        let approval_ratio = self.current_approval_ratio();

        if elapsed < total && approval_ratio >= threshold {
            self.status = ProposalStatus::Accepted;
        } else if elapsed >= total {
            self.status = ProposalStatus::Rejected;
        }
    }

    /// Attempts to extend the voting window if conditions are met.
    ///
    /// Extension occurs when the proposal is both near the approval threshold
    /// and near the time expiry, allowing for last-minute consensus building.
    ///
    /// # Arguments
    /// * `now` - Current timestamp
    /// * `extension_seconds` - How many seconds to extend by
    /// * `threshold_proximity` - Ratio (0.0-1.0) of threshold that triggers extension
    /// * `time_proximity` - Ratio (0.0-1.0) of time elapsed that triggers extension
    pub fn extend_window(
        &mut self,
        now: DateTime<Utc>,
        extension_seconds: u64,
        threshold_proximity: f64,
        time_proximity: f64,
    ) {
        if self.status != ProposalStatus::Pending {
            return;
        }

        let elapsed = self.voting_window.elapsed(now);
        let total = self.voting_window.total_duration();
        let threshold = threshold_calc(&self.threshold_model, elapsed, total);
        let approval_ratio = self.current_approval_ratio();

        let near_threshold = approval_ratio >= threshold * threshold_proximity;
        let near_expiry = elapsed as f64 >= total as f64 * time_proximity;

        if near_threshold && near_expiry {
            self.voting_window.extend(extension_seconds);
        }
    }

    /// Calculates the current approval ratio based on weighted votes.
    ///
    /// The approval ratio is calculated as:
    /// `yes_weight / (yes_weight + no_weight)`
    ///
    /// Abstain votes are not counted in the ratio calculation.
    ///
    /// # Returns
    /// The approval ratio as a value between 0.0 and 1.0, or 0.0 if no votes
    pub fn current_approval_ratio(&self) -> f64 {
        let mut yes_weight = 0.0;
        let mut total_weight = 0.0;

        for vote in &self.votes {
            let weight = calculate_vote_weight(
                vote,
                self.voting_window.start_time,
                self.voting_window.total_duration(),
                &self.decay_model,
            );

            match vote.choice {
                VoteChoice::Yes => {
                    yes_weight += weight;
                    total_weight += weight;
                }
                VoteChoice::No => {
                    total_weight += weight;
                }
                VoteChoice::Abstain => {}
            }
        }

        if total_weight > 0.0 {
            yes_weight / total_weight
        } else {
            0.0
        }
    }
}
