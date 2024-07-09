mod blockchain;

use blockchain::BlockchainListener;
use clap::Parser;
use dotenv::dotenv;
use log::{error, info};
use std::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "wss://ethereum-rpc.publicnode.com")]
    node_url: String,

    #[arg(
        short,
        long,
        default_value = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"
    )]
    contract_address: String,

    #[arg(short, long, default_value = "./usdc_abi.json")]
    abi_path: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    let args = Args::parse();

    info!("Starting blockchain event daeamon");
    info!("Connecting to node: {}", args.node_url);

    // let contract_abi = fs::read_to_string(args.abi_path)?;

    match BlockchainListener::new(&args.node_url, &args.contract_address).await {
        Ok(listener) => {
            if let Err(e) = listener.listen_for_events().await {
                error!("Error listening for event: {:?}", e);
            }
        }
        Err(e) => error!("Failed to create Blockchain Listener: {:?}", e),
    }

    info!("Blockchain event daemon stopped!");
    Ok(())
}
