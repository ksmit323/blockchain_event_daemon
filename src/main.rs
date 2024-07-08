use clap::Parser;
use dotenv::dotenv;
use log::info;

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

    // TODO: Set up blockchain connection and event listener

    info!("Blockchain event daemon stopped!");
    Ok(())
}
