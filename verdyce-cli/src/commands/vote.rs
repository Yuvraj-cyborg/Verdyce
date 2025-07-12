use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::str::FromStr;
use verdyce_core::models::proposal::Proposal;
use verdyce_core::models::vote::{Vote, VoteChoice};
use crate::redis;

pub async fn cast_vote(
    proposal_id: &str,
    validator_id: &str,
    choice_str: &str,
    revision: u64,
    reason: Option<String>,
    timestamp_str: Option<String>,
) {
    let proposal_uuid = match Uuid::parse_str(proposal_id) {
        Ok(uuid) => uuid,
        Err(_) => {
            eprintln!("❌ Invalid proposal ID.");
            return;
        }
    };

    let validator_uuid = match Uuid::parse_str(validator_id) {
        Ok(uuid) => uuid,
        Err(_) => {
            eprintln!("❌ Invalid validator ID (UUID required).");
            return;
        }
    };

    let choice = match choice_str.to_lowercase().as_str() {
        "yes" => VoteChoice::Yes,
        "no" => VoteChoice::No,
        "abstain" => VoteChoice::Abstain,
        _ => {
            eprintln!("❌ Invalid vote choice. Use: yes | no | abstain");
            return;
        }
    };

    let timestamp = match timestamp_str {
        Some(ts) => match DateTime::from_str(&ts) {
            Ok(parsed) => parsed,
            Err(_) => {
                eprintln!("❌ Invalid timestamp format. Use RFC3339 format.");
                return;
            }
        },
        None => Utc::now(),
    };

    let key = format!("proposal:{}", proposal_uuid);
    let Some(mut proposal) = redis::get_json::<Proposal>(&key).await.unwrap_or(None) else {
        eprintln!("❌ Proposal not found.");
        return;
    };

    let elapsed = proposal.voting_window.elapsed(timestamp);
    if elapsed > proposal.voting_window.total_duration() + proposal.voting_window.grace_period {
        eprintln!("⏱️ Voting window has ended.");
        return;
    }

    let vote = Vote {
        validator_id: validator_uuid,
        choice,
        timestamp,
        revision,
        reason,
    };

    proposal.add_vote(vote);

    if let Err(e) = redis::save_json(&key, &proposal).await {
        eprintln!("❌ Failed to save updated proposal: {:?}", e);
        return;
    }

    println!("✅ Vote by validator {} recorded.", validator_uuid);
}
