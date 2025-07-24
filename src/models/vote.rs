//! # Vote Model
//!
//! Defines vote structures and weight calculation logic for the consensus system.

use crate::decay::{DecayModel, weight_calc};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Represents the choice made in a vote.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VoteChoice {
    /// Vote in favor of the proposal
    Yes,
    /// Vote against the proposal
    No,
    /// Abstain from voting (doesn't count toward approval ratio)
    Abstain,
}

/// Represents a single vote cast by a validator.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Vote {
    /// Unique identifier of the validator casting the vote
    pub validator_id: Uuid,
    /// The choice made in this vote
    pub choice: VoteChoice,
    /// When this vote was cast
    pub timestamp: DateTime<Utc>,
    /// Number of times this validator has changed their vote (0 = first vote)
    pub revision: u64,
    /// Optional reason for the vote or vote change
    pub reason: Option<String>,
}

/// Calculates the effective weight of a vote based on timing and revision history.
///
/// The weight is determined by:
/// 1. Base weight from the decay model (decreases over time)
/// 2. Revision penalty (decreases with each vote change)
/// 3. Minimum floor of 0.1 to ensure all votes have some weight
///
/// # Arguments
/// * `vote` - The vote to calculate weight for
/// * `proposal_start` - When the proposal's voting period began
/// * `total_time` - Total duration of the voting period in seconds
/// * `decay_model` - The decay model to use for time-based weight reduction
///
/// # Returns
/// The effective weight of the vote (between 0.1 and 1.0)
///
/// # Examples
/// ```
/// use chrono::Utc;
/// use verdyce_core::models::vote::{Vote, VoteChoice, calculate_vote_weight};
/// use verdyce_core::decay::DecayModel;
/// use uuid::Uuid;
///
/// let start = Utc::now();
/// let vote = Vote {
///     validator_id: Uuid::new_v4(),
///     choice: VoteChoice::Yes,
///     timestamp: start,
///     revision: 0,
///     reason: None,
/// };
/// let weight = calculate_vote_weight(&vote, start, 3600, &DecayModel::Linear);
/// assert!((weight - 1.0).abs() < 0.01); // Full weight at start
/// ```
pub fn calculate_vote_weight(
    vote: &Vote,
    proposal_start: DateTime<Utc>,
    total_time: u64,
    decay_model: &DecayModel,
) -> f64 {
    let time_elapsed = (vote.timestamp - proposal_start).num_seconds().max(0) as u64;
    let base_weight = weight_calc(decay_model, time_elapsed, total_time);

    // Revision penalty: weight / (1 + revisions)^2
    let penalty = (1 + vote.revision).pow(2) as f64;
    let penalized_weight = base_weight / penalty;
    penalized_weight.max(0.1)
}
