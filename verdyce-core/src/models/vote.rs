use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::decay::{DecayModel, weight_calc};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VoteChoice {
    Yes,
    No,
    Abstain,
}
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Vote {
    pub validator_id: Uuid,
    pub choice: VoteChoice,
    pub timestamp: DateTime<Utc>,
    pub revision: u64,
    pub reason: Option<String>
}

pub fn calculate_vote_weight(vote: &Vote, proposal_start: DateTime<Utc>, total_time: u64, decay_model: &DecayModel) -> f64 {
    let time_elapsed = (vote.timestamp - proposal_start).num_seconds().max(0) as u64;
    let base_weight = weight_calc(decay_model, time_elapsed, total_time);
    
    // Revision penalty: weight / (1 + revisions)^2
    let penalty = (1 + vote.revision).pow(2) as f64;
    let penalized_weight = base_weight / penalty;
    penalized_weight.max(0.1)
}
