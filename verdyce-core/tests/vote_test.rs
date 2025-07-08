use chrono::{Utc, Duration};
use uuid::Uuid;
use verdyce_core::decay::DecayModel;
use verdyce_core::models::vote::{Vote, VoteChoice, calculate_vote_weight};

#[test]
fn test_vote_at_start_no_revision() {
    let now = Utc::now();
    let vote = Vote {
        validator_id: Uuid::new_v4(),
        choice: VoteChoice::Yes,
        timestamp: now,
        revision: 0,
        reason: None,
    };

    let model = DecayModel::Linear;
    let weight = calculate_vote_weight(&vote, now, 1800, &model);
    assert!((weight - 1.0).abs() < 0.01);
}


#[test]
fn test_vote_halfway_with_revision() {
    let now = Utc::now();
    let timestamp = now - Duration::seconds(900); // 900s = halfway
    let vote = Vote {
        validator_id: Uuid::new_v4(),
        choice: VoteChoice::No,
        timestamp,
        revision: 1,
        reason: Some("Changed mind".to_string()),
    };
    let model = DecayModel::Linear;
    let weight = calculate_vote_weight(&vote, now, 1800, &model);
    // Without penalty: 0.5, penalty: /4 => 0.125
    assert!((weight - 0.125).abs() < 0.01);
}

#[test]
fn test_vote_near_expiry_high_revision() {
    let now = Utc::now();
    let timestamp = now - Duration::seconds(1700); // near end
    let vote = Vote {
        validator_id: Uuid::new_v4(),
        choice: VoteChoice::Abstain,
        timestamp,
        revision: 3,
        reason: Some("Unstable".to_string()),
    };

    let model = DecayModel::Linear;
    let weight = calculate_vote_weight(&vote, now, 1800, &model);
    // Without penalty: ~0.055, penalty: /16 = ~0.0034375, floored to 0.1
    assert!((weight - 0.1).abs() < 0.001);
}