use clap::{Parser, Subcommand};

pub mod commands;
pub use commands::*;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {   

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    CreateMint {
        #[arg(short, long, default_value = "https://api.devnet.solana.com")]
        rpc_url: String,

        #[arg(short, long)]
        keypair: String
    },

    MintTo {
        #[arg(short, long, default_value = "https://api.devnet.solana.com")]
        rpc_url: String,

        #[arg(short, long)]
        keypair: String,

        #[arg(short, long)]
        mint: String,

        #[arg(short, long)]
        recipient: String,

        #[arg(short, long)]
        amount: u64
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    match args.command {
        Commands::CreateMint { rpc_url, keypair } => {
            create_mint(&rpc_url, &keypair)
                .unwrap();
                
        }

        Commands::MintTo {
            rpc_url, 
            keypair, 
            mint, 
            recipient, 
            amount 
        } => {
            mint_tokens(
                &rpc_url, 
                &keypair, 
                &mint, 
                &recipient, 
                amount
            ).unwrap();
        }
    }
}