use verdyce_core::models::proposal::*;
use verdyce_core::models::vote::*;
use verdyce_core::decay::DecayModel;
use verdyce_core::threshold::ThresholdModel;

use chrono::{Duration, Utc};
use uuid::Uuid;

fn dummy_vote(choice: VoteChoice, timestamp_offset_secs: i64, revision: u64) -> Vote {
    Vote {
        validator_id: Uuid::new_v4(),
        choice,
        timestamp: Utc::now() - Duration::seconds(timestamp_offset_secs),
        revision,
        reason: None,
    }
}

#[test]
fn test_proposal_accepts_with_enough_yes_votes() {
    let now = Utc::now();

    let mut proposal = Proposal::new(
        "Test".to_string(),
        "Desc".to_string(),
        600,
        DecayModel::Linear,
        ThresholdModel::Linear(0.0, 0.5),
    );

    proposal.add_vote(dummy_vote(VoteChoice::Yes, 0, 0));
    proposal.add_vote(dummy_vote(VoteChoice::Yes, 0, 0));
    proposal.add_vote(dummy_vote(VoteChoice::No, 0, 0));

    proposal.evaluate(now + Duration::seconds(60));

    assert_eq!(proposal.status, ProposalStatus::Accepted);
}

#[test]
fn test_proposal_expires_if_not_enough_votes() {
    let now = Utc::now();

    let mut proposal = Proposal::new(
        "Test".to_string(),
        "Desc".to_string(),
        300,
        DecayModel::Linear,
        ThresholdModel::Linear(0.0, 0.7),
    );

    proposal.voting_window.start_time = now - Duration::seconds(600);

    proposal.add_vote(dummy_vote(VoteChoice::Yes, 0, 0));
    proposal.add_vote(dummy_vote(VoteChoice::No, 0, 0));

    proposal.evaluate(now);

    assert_eq!(proposal.status, ProposalStatus::Expired);
}


#[test]
fn test_proposal_stays_pending_if_not_enough_yes_yet() {
    let now = Utc::now();

    let mut proposal = Proposal::new(
        "Test".to_string(),
        "Desc".to_string(),
        500,
        DecayModel::Linear,
        ThresholdModel::Linear(0.01, 0.5),
    );

    proposal.add_vote(dummy_vote(VoteChoice::Yes, 0, 0));
    proposal.add_vote(dummy_vote(VoteChoice::No, 0, 0));

    proposal.evaluate(now + Duration::seconds(100));

    assert_eq!(proposal.status, ProposalStatus::Pending);
}
#[test]
fn test_extend_when_near_threshold_and_time() {
    let now = Utc::now();
    let mut proposal = Proposal::new(
        "Extend?".into(),
        "testing...".into(),
        100,
        DecayModel::Linear,
        ThresholdModel::Linear(0.0, 0.6),
    );

    proposal.voting_window.start_time = now - Duration::seconds(91); 
    proposal.add_vote(Vote {
        validator_id: Uuid::new_v4(),
        choice: VoteChoice::Yes,
        timestamp: now,
        revision: 0,
        reason: None,
    });

    proposal.extend_window(now, 30, 0.9, 0.9);
    assert_eq!(proposal.voting_window.extended_by, 30);
}
