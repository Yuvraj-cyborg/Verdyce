mod redis;
mod commands;

use clap::{Parser, Subcommand};
use commands::{new_proposal, vote, evaluate};

#[derive(Parser)]
#[command(name = "verdyce")]
#[command(about = "Verdyce Time-Decay Voting CLI", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    NewProposal {
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        description: String,
        #[arg(short, long)]
        duration: u64,
    },
    Vote {
        #[arg(short, long)]
        proposal_id: String,

        #[arg(short, long)]
        validator_id: String,

        #[arg(short, long)]
        choice: String,

        #[arg(short, long, default_value = "0")]
        revision: u64,

        #[arg(short, long)]
        reason: Option<String>,

        #[arg(short, long)]
        timestamp: Option<String>,
    },
    Evaluate {
    #[arg(short, long)]
    id: String,
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok(); 
    let cli = Cli::parse();

    match cli.command {
        Commands::NewProposal { title, description, duration } => {
            new_proposal::new_proposal(&title, &description, duration).await;
        }
        Commands::Vote {
            proposal_id,
            validator_id,
            choice,
            revision,
            reason,
            timestamp,
        } => {
            vote::cast_vote(&proposal_id, &validator_id, &choice, revision, reason, timestamp).await;
        }
        Commands::Evaluate { id } => {
        evaluate::evaluate_proposal(&id).await;
        }
    }
}
