use log::{error, info};
use web3::futures::StreamExt;
use web3::transports::WebSocket;
use web3::types::{BlockNumber, FilterBuilder, H160};
use web3::Web3;

pub struct BlockchainListener {
    web3: Web3<WebSocket>,
}

impl BlockchainListener {
    pub async fn new(node_url: &str) -> Result<Self, web3::Error> {
        let transport = WebSocket::new(node_url).await?;
        let web3 = Web3::new(transport);
        Ok(Self { web3 })
    }

    pub async fn listen_for_events(&self) -> Result<(), web3::Error> {
        let filter = FilterBuilder::default()
            .from_block(BlockNumber::Latest)
            .address(vec![H160::zero()]) //* Will need to replace with an actual contract address */
            .build();

        let mut stream = self.web3.eth_subscribe().subscribe_logs(filter).await?;

        info!("Listening for blockchain events...");

        while let Some(log) = stream.next().await {
            match log {
                Ok(log) => {
                    info!("Received Event: {:?}", log)
                }
                Err(e) => error!("Error receiving event: {:?}", e),
            }
        }
        Ok(())
    }
}
