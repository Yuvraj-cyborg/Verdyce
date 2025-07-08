use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::models::proposal::{Proposal, ProposalStatus};
use crate::models::vote::Vote;

pub struct Engine {
    pub proposals: Vec<Proposal>,
}

impl Engine {
    pub fn new() -> Self {
        Self {
            proposals: Vec::new(),
        }
    }

    pub fn add_proposal(&mut self, proposal: Proposal) {
        self.proposals.push(proposal);
    }

    pub fn cast_vote(&mut self, proposal_id: Uuid, vote: Vote) -> bool {
        if let Some(proposal) = self.proposals.iter_mut().find(|p| p.id == proposal_id) {
            if proposal.status == ProposalStatus::Pending {
                proposal.add_vote(vote);
                return true;
            }
        }
        false
    }

    pub fn evaluate_all(&mut self, now: DateTime<Utc>) {
        for proposal in &mut self.proposals {
            proposal.evaluate(now);
        }
    }

    pub fn maybe_extend_all(
        &mut self,
        now: DateTime<Utc>,
        extension_seconds: u64,
        threshold_proximity: f64,
        time_proximity: f64,
    ) {
        for proposal in &mut self.proposals {
            proposal.extend_window(
                now,
                extension_seconds,
                threshold_proximity,
                time_proximity,
            );
        }
    }

    pub fn get_active_proposals(&self) -> Vec<&Proposal> {
        self.proposals
            .iter()
            .filter(|p| p.status == ProposalStatus::Pending)
            .collect()
    }

    pub fn get_finalized(&self) -> Vec<&Proposal> {
        self.proposals
            .iter()
            .filter(|p| {
                matches!(
                    p.status,
                    ProposalStatus::Accepted
                        | ProposalStatus::Rejected
                        | ProposalStatus::Expired
                )
            })
            .collect()
    }

    pub fn get_proposal(&self, proposal_id: Uuid) -> Option<&Proposal> {
        self.proposals.iter().find(|p| p.id == proposal_id)
    }
}
