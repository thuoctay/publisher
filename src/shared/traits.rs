use async_trait::async_trait;
use crate::domain::models::transaction::Transaction;
use alloy::primitives::{Address, U256};
use crate::error::AppResult;

#[async_trait]
pub trait TransactionRepository {
    async fn fetch_new_transactions(&self, since: chrono::DateTime<chrono::Utc>) -> AppResult<Vec<Transaction>>;
}

#[async_trait]
pub trait ProcessedJobsTracker {
    async fn is_processed(&self, record_id: i64) -> AppResult<bool>;
    async fn mark_pending(&self, record_id: i64) -> AppResult<()>;
    async fn mark_sent(&self, record_id: i64, tx_hash: &str) -> AppResult<()>;
    async fn mark_failed(&self, record_id: i64) -> AppResult<()>;
}

#[async_trait]
pub trait BlockchainService {
    async fn send_transaction(&self, to: Address, value: U256) -> AppResult<[u8; 32]>;
}

#[async_trait]
pub trait TransactionProcessor {
    async fn process_transaction(&self, transaction: &Transaction) -> AppResult<()>;
}

#[async_trait]
pub trait AppService {
    async fn start(&mut self) -> AppResult<()>;
    async fn stop(&self) -> AppResult<()>;
} 