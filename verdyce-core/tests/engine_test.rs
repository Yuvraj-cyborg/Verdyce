use verdyce_core::models::{
    proposal::{Proposal, ProposalStatus},
    vote::{Vote, VoteChoice},
};
use verdyce_core::engine::Engine;
use verdyce_core::decay::DecayModel;
use verdyce_core::threshold::ThresholdModel;
use chrono::{Utc, Duration};
use uuid::Uuid;

fn sample_proposal() -> Proposal {
    Proposal::new(
        "Test Proposal".into(),
        "Description".into(),
        60, 
        DecayModel::Linear,
        ThresholdModel::Linear(0.5, 0.0),
    )
}

fn sample_vote(choice: VoteChoice, seconds_ago: i64, revision: u64) -> Vote {
    Vote {
        validator_id: Uuid::new_v4(),
        choice,
        timestamp: Utc::now() - Duration::seconds(seconds_ago),
        revision,
        reason: Some("test".into()),
    }
}

#[test]
fn test_add_and_get_proposal() {
    let mut engine = Engine::new();
    let proposal = sample_proposal();
    let id = proposal.id;

    engine.add_proposal(proposal);
    let retrieved = engine.get_proposal(id);

    assert!(retrieved.is_some());
    assert_eq!(retrieved.unwrap().id, id);
}

#[test]
fn test_cast_vote_success() {
    let mut engine = Engine::new();
    let proposal = sample_proposal();
    let id = proposal.id;
    engine.add_proposal(proposal);

    let vote = sample_vote(VoteChoice::Yes, 10, 0);
    let success = engine.cast_vote(id, vote);

    assert!(success);
    assert_eq!(engine.get_proposal(id).unwrap().votes.len(), 1);
}

#[test]
fn test_cast_vote_failure_invalid_id() {
    let mut engine = Engine::new();
    let vote = sample_vote(VoteChoice::Yes, 10, 0);

    let fake_id = Uuid::new_v4();
    let result = engine.cast_vote(fake_id, vote);

    assert!(!result);
}

#[test]
fn test_evaluate_accept_proposal() {
    let mut engine = Engine::new();
    let mut proposal = sample_proposal();
    let id = proposal.id;

    let vote = sample_vote(VoteChoice::Yes, 5, 0);
    proposal.add_vote(vote);
    engine.add_proposal(proposal);

    let now = Utc::now();
    engine.evaluate_all(now);

    let status = engine.get_proposal(id).unwrap().status.clone();
    assert_eq!(status, ProposalStatus::Accepted);
}

#[test]
fn test_evaluate_expired_proposal() {
    let mut engine = Engine::new();
    let mut proposal = sample_proposal();
    proposal.voting_window.start_time = Utc::now() - Duration::seconds(120);
    let id = proposal.id;

    engine.add_proposal(proposal);
    engine.evaluate_all(Utc::now());

    let status = engine.get_proposal(id).unwrap().status.clone();
    assert_eq!(status, ProposalStatus::Expired);
}

#[test]
fn test_maybe_extend_all() {
    let mut engine = Engine::new();
    let mut proposal = Proposal::new(
        "Test".into(),
        "Should Extend".into(),
        100,
        DecayModel::Linear,
        ThresholdModel::Linear(0.0, 0.6),
    );

    proposal.voting_window.start_time = Utc::now() - Duration::seconds(91);
    proposal.add_vote(Vote {
        validator_id: Uuid::new_v4(),
        choice: VoteChoice::Yes,
        timestamp: Utc::now(),
        revision: 0,
        reason: None,
    });

    let id = proposal.id;
    engine.add_proposal(proposal);
    engine.maybe_extend_all(Utc::now(), 30, 0.9, 0.9);

    let proposal = engine.get_proposal(id).unwrap();
    assert_eq!(proposal.voting_window.extended_by, 30);
}
