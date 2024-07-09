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

sol!(
    #[allow(missing_docs)]
    #[sol(rpc)]
    USDC,
    "src/abi/usdc_abi.json",
);

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

        // Create the provider
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
            .event("Transfer(address,address,uint256)")
            .from_block(BlockNumberOrTag::Latest);

        // Subscribe to logs
        let sub = self.provider.subscribe_logs(&filter).await?;
        let mut stream = sub.into_stream();

        info!("Listening for blockchain events...");

        while let Some(log) = stream.next().await {

            // // Match on topic 0, the hash of the signature of the event.            
            // match log.topic0() {
            //     // Match the `Transfer(address,address,uint256)` event.
            //     Some(&USDC::Transfer::SIGNATURE_HASH) => {
            //         let USDC::Transfer { from, to, value } = log.log_decode()?.inner.data;
            //         info!("Transfer Event: From: {from}, To: {to}, Amount; {value}");
            //     }
            //     _ => (),
            // }

            info!("Received event 111111: {log:?}");
            info!("Received event 222222: {:?}", log.topic0());
            info!("Received event 333333: {:?}", log.topics());
        }

        Ok(())
    }

    // pub async fn process_event(&self, log: Log) -> Result<(), Box<dyn std::error::Error>> {
    //     // Need to convert web3::types::Log to ethers::abi::RawLog
    //     let raw_log = RawLog {
    //         topics: log.topics,
    //         data: log.data.0,
    //     };

    //     let event = self
    //         .contract_abi
    //         .event("Transfer")?
    //         .parse_log(raw_log.into())?;

    //     let from: EthersH160 = event.params[0].value.clone().into_address().unwrap();
    //     let to: EthersH160 = event.params[1].value.clone().into_address().unwrap();
    //     let amount: U256 = event.params[2].value.clone().into_uint().unwrap();

    //     info!(
    //         "Transfer Event: From: {:?}, To: {:?}, Amount: {}",
    //         from, to, amount
    //     );

    //     // TODO: Add custom logic here:
    //     // - Store the event in a database
    //     // - Trigger notifications
    //     // - update application state

    //     Ok(())
    // }
}
