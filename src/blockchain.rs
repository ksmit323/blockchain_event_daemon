use web3::transports::WebSocket;
use web3::Web3;
use web3::types::{FilterBuilder, H160, BlockNumber};
use log::{info, error};

pub struct BlockchainListener {
    web3: Web3<WebSocket>,
}

impl BlockchainListener {
    
}