use verdyce_core::models::proposal::Proposal;
use verdyce_core::decay::DecayModel;
use verdyce_core::threshold::ThresholdModel;
use crate::redis;

pub async fn new_proposal(title: &str, description: &str, duration_secs: u64) {
    let proposal = Proposal::new(
        title.to_string(),
        description.to_string(),
        duration_secs,
        DecayModel::Exponential(0.1),
        ThresholdModel::Linear(0.0, 0.5),
    );

    let key = format!("proposal:{}", proposal.id);
    redis::save_json(&key, &proposal)
        .await
        .expect("Failed to save proposal");

    println!("\n📝 Proposal Created:");
    println!("  ID        : {}", proposal.id);
    println!("  Title     : {}", title);
    println!("  Duration  : {} seconds", duration_secs);
    println!("  Expires At: {}", proposal.created_at + chrono::Duration::seconds(duration_secs as i64));
}
