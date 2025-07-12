use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::decay::DecayModel;
use crate::models::vote::{Vote, VoteChoice, calculate_vote_weight};
use crate::threshold::{ThresholdModel, threshold_calc};
use crate::window::VotingWindow;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProposalStatus {
    Pending,
    Accepted,
    Rejected,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub votes: Vec<Vote>,
    pub status: ProposalStatus,
    pub voting_window: VotingWindow,
    pub decay_model: DecayModel,
    pub threshold_model: ThresholdModel,
}

impl Proposal {
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

    pub fn add_vote(&mut self, vote: Vote) {
        self.votes.push(vote);
    }

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
