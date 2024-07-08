mod blockchain;

use blockchain::BlockchainListener;
use clap::Parser;
use dotenv::dotenv;
use log::{info, error};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "")]
    node_url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Initialize logger
    env_logger::init();

    // Parse command-line arguments
    let args = Args::parse();

    info!("Starting blockchain event daeamon");
    info!("Connecting to node: {}", args.node_url);

    match BlockchainListener::new(&args.node_url).await {
        Ok(listener) => {
            if let Err(e) = listener.listen_for_events().await {
                error!("Error listening for event: {:?}", e);
            }
        },
        Err(e) => error!("Failed to create Blockchain Listener: {:?}", e),
    }

    info!("Blockchain event daemon stopped!");
    Ok(())
}
