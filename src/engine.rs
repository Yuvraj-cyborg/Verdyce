//! # Consensus Engine
//!
//! The main coordinator for the Verdyce consensus system. Manages proposals,
//! votes, and evaluation logic.

use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::models::proposal::{Proposal, ProposalStatus};
use crate::models::vote::Vote;

/// The main consensus engine that coordinates proposals and voting.
///
/// The engine maintains a collection of proposals and provides methods to:
/// - Add new proposals
/// - Cast votes on proposals
/// - Evaluate proposal outcomes
/// - Extend voting windows when appropriate
pub struct Engine {
    /// Collection of all proposals managed by this engine
    pub proposals: Vec<Proposal>,
}

impl Default for Engine {
    fn default() -> Self {
        Self::new()
    }
}

impl Engine {
    pub fn new() -> Self {
        Self {
            proposals: Vec::new(),
        }
    }

    /// Adds a new proposal to the engine.
    ///
    /// # Arguments
    /// * `proposal` - The proposal to add
    pub fn add_proposal(&mut self, proposal: Proposal) {
        self.proposals.push(proposal);
    }

    /// Attempts to cast a vote on a proposal.
    ///
    /// # Arguments
    /// * `proposal_id` - UUID of the proposal to vote on
    /// * `vote` - The vote to cast
    ///
    /// # Returns
    /// `true` if the vote was successfully cast, `false` if the proposal
    /// doesn't exist or is not in pending status
    pub fn cast_vote(&mut self, proposal_id: Uuid, vote: Vote) -> bool {
        if let Some(proposal) = self.proposals.iter_mut().find(|p| p.id == proposal_id)
            && proposal.status == ProposalStatus::Pending
        {
            proposal.add_vote(vote);
            return true;
        }
        false
    }

    /// Evaluates all proposals to determine their current status.
    ///
    /// This checks each proposal against its threshold and time constraints
    /// to determine if it should be accepted, rejected, or expired.
    ///
    /// # Arguments
    /// * `now` - Current timestamp for evaluation
    pub fn evaluate_all(&mut self, now: DateTime<Utc>) {
        for proposal in &mut self.proposals {
            proposal.evaluate(now);
        }
    }

    /// Attempts to extend voting windows for all proposals that meet extension criteria.
    ///
    /// A proposal's window may be extended if it's both near the approval threshold
    /// and near the time expiry.
    ///
    /// # Arguments
    /// * `now` - Current timestamp
    /// * `extension_seconds` - How many seconds to extend by
    /// * `threshold_proximity` - Ratio (0.0-1.0) of how close to threshold to trigger extension
    /// * `time_proximity` - Ratio (0.0-1.0) of how close to expiry to trigger extension
    pub fn maybe_extend_all(
        &mut self,
        now: DateTime<Utc>,
        extension_seconds: u64,
        threshold_proximity: f64,
        time_proximity: f64,
    ) {
        for proposal in &mut self.proposals {
            proposal.extend_window(now, extension_seconds, threshold_proximity, time_proximity);
        }
    }

    /// Returns all proposals that are currently pending (accepting votes).
    pub fn get_active_proposals(&self) -> Vec<&Proposal> {
        self.proposals
            .iter()
            .filter(|p| p.status == ProposalStatus::Pending)
            .collect()
    }

    /// Returns all proposals that have reached a final state.
    pub fn get_finalized(&self) -> Vec<&Proposal> {
        self.proposals
            .iter()
            .filter(|p| {
                matches!(
                    p.status,
                    ProposalStatus::Accepted | ProposalStatus::Rejected | ProposalStatus::Expired
                )
            })
            .collect()
    }

    /// Retrieves a specific proposal by ID.
    ///
    /// # Arguments
    /// * `proposal_id` - UUID of the proposal to retrieve
    ///
    /// # Returns
    /// `Some(&Proposal)` if found, `None` otherwise
    pub fn get_proposal(&self, proposal_id: Uuid) -> Option<&Proposal> {
        self.proposals.iter().find(|p| p.id == proposal_id)
    }
}
