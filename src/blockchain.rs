use eyre::Result;
use futures_util::StreamExt;
use log::{error, info};
use std::str::FromStr;

use alloy::{
    primitives::{address, Address, U256},
    providers::{Provider, ProviderBuilder, RootProvider, WsConnect},
    pubsub::PubSubFrontend,
    rpc::types::{BlockNumberOrTag, Filter, TransactionRequest},
    signers::local::PrivateKeySigner,
    sol,
    sol_types::SolEvent,
};

pub struct BlockchainListener {
    provider: RootProvider<PubSubFrontend>,
    contract_address: Address,
    // contract_abi: Abi,
}

impl BlockchainListener {
    pub async fn new(
        node_url: &str,
        contract_address: &str,
        // json_abi: &str,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let ws = WsConnect::new(node_url);
        let provider = ProviderBuilder::new().on_ws(ws).await?;
        let contract_address = contract_address.parse()?;

        // let contract_abi: Abi = serde_json::from_str(json_abi)?;

        Ok(Self {
            provider,
            contract_address,
            // contract_abi,
        })
    }

    pub async fn listen_for_events(&self) -> Result<()> {
        // Create a filter to watch for all contract events
        let filter = Filter::new()
            .address(self.contract_address)
            .from_block(BlockNumberOrTag::Latest);

        // Subscribe to logs
        let sub = self.provider.subscribe_logs(&filter).await?;
        let mut stream = sub.into_stream();

        info!("Listening for blockchain events...");

        while let Some(log) = stream.next().await {
            info!("Received event: {:?}", log);
        }

        Ok(())
    }
}
