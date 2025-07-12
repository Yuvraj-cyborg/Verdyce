use verdyce_core::models::proposal::Proposal;
use chrono::Utc;
use crate::redis;

pub async fn evaluate_proposal(id: &str) {
    let key = format!("proposal:{}", id);

    match redis::get_json::<Proposal>(&key).await {
        Ok(Some(mut proposal)) => {
            let now = Utc::now();
            proposal.evaluate(now);

            redis::save_json(&key, &proposal).await.expect("Failed to save evaluated proposal");

            println!("\n📊 Proposal Evaluation Complete:");
            println!("  ID     : {}", proposal.id);
            println!("  Status : {:?}", proposal.status);
        }
        Ok(None) => {
            println!("❌ Proposal with ID '{}' not found", id);
        }
        Err(e) => {
            println!("🚨 Redis error: {}", e);
        }
    }
}
