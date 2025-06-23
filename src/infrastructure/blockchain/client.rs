use crate::{
    error::AppResult,
    shared::traits::BlockchainService,
};
use alloy::primitives::{Address, U256};
use async_trait::async_trait;
use std::time::Duration;
use tracing::info;


#[derive(Clone)]
pub struct BlockchainClient;

impl BlockchainClient {
    /// Creates a new simulated blockchain client.
    /// The parameters are kept to maintain a consistent signature with the real client.
    pub fn new(_rpc_url: &str, _private_key: &str) -> AppResult<Self> {
        info!("Initializing SIMULATED Blockchain Client (v1.0 compatible)");
        Ok(Self)
    }
}

#[async_trait]
impl BlockchainService for BlockchainClient {
    /// Simulates sending a transaction and returns a fake transaction hash.
    async fn send_transaction(&self, to: Address, value: U256) -> AppResult<[u8; 32]> {
        info!("SIMULATING sending transaction: to={}, value={}", to, value);

        // Simulate network delay
        tokio::time::sleep(Duration::from_millis(750)).await;

        // Create a fake, zeroed-out transaction hash
        let fake_tx_hash = [0u8; 32];
        info!(
            "SIMULATION successful. Fake tx_hash: 0x{}",
            hex::encode(fake_tx_hash)
        );

        Ok(fake_tx_hash)
    }
}
