use crate::domain::models::transaction::{Transaction, TransactionPayload};
use crate::shared::traits::{BlockchainService, ProcessedJobsTracker as ProcessedJobsTrackerTrait, TransactionProcessor, TransactionRepository};
use alloy::primitives::{Address, U256};
use async_trait::async_trait;
use sqlx::PgPool;
use std::{str::FromStr, sync::Arc};
use tracing::{error, info, warn};
use crate::error::AppResult;

/// Postgres-based transaction repository
pub struct PostgresTransactionRepository {
    pool: PgPool,
}

impl PostgresTransactionRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TransactionRepository for PostgresTransactionRepository {
    async fn fetch_new_transactions(&self, since: chrono::DateTime<chrono::Utc>) -> AppResult<Vec<Transaction>> {
        let rows = sqlx::query!(
            r#"
            SELECT id, created_at, payload, status
            FROM transactions
            WHERE created_at > $1
            ORDER BY created_at
            "#,
            since
        )
        .fetch_all(&self.pool)
        .await?;

        let transactions = rows
            .into_iter()
            .map(|row| Transaction {
                id: row.id,
                created_at: row.created_at.unwrap_or_else(|| chrono::Utc::now()),
                payload: row.payload,
                status: row.status.unwrap_or_else(|| "pending".to_string()),
            })
            .collect();

        Ok(transactions)
    }
}

/// Transaction processor that handles the business logic
pub struct TransactionProcessorService {
    processed_jobs_tracker: Arc<dyn ProcessedJobsTrackerTrait + Send + Sync>,
    blockchain_service: Arc<dyn BlockchainService + Send + Sync>,
}

impl TransactionProcessorService {
    pub fn new(
        processed_jobs_tracker: Arc<dyn ProcessedJobsTrackerTrait + Send + Sync>,
        blockchain_service: Arc<dyn BlockchainService + Send + Sync>,
    ) -> Self {
        Self {
            processed_jobs_tracker,
            blockchain_service,
        }
    }
}

#[async_trait]
impl TransactionProcessor for TransactionProcessorService {
    async fn process_transaction(&self, transaction: &Transaction) -> AppResult<()> {
        // Check if already processed
        if self.processed_jobs_tracker.is_processed(transaction.id as i64).await? {
            warn!("Transaction ID {} has already been processed. Skipping.", transaction.id);
            return Ok(());
        }

        info!(
            "Processing new transaction: id={}, created_at={:?}",
            transaction.id,
            transaction.created_at.to_rfc3339()
        );

        // Mark as pending before processing
        self.processed_jobs_tracker.mark_pending(transaction.id as i64).await?;

        // Parse the payload to get transaction details
        let payload: TransactionPayload = serde_json::from_value(transaction.payload.clone())?;
        let to_address = Address::from_str(&payload.to)
            .map_err(|e| crate::error::AppError::Validation(format!("Invalid address: {}", e)))?;
        let value = U256::from_str_radix(&payload.amount, 10)
            .map_err(|e| crate::error::AppError::Validation(format!("Invalid amount: {}", e)))?;

        // Send the transaction
        match self.blockchain_service.send_transaction(to_address, value).await {
            Ok(tx_hash) => {
                // Mark as sent with transaction hash
                let tx_hash_hex = format!("0x{}", hex::encode(tx_hash));
                self.processed_jobs_tracker.mark_sent(transaction.id as i64, &tx_hash_hex).await?;
            }
            Err(e) => {
                error!("Failed to send transaction for record {}: {}", transaction.id, e);
                self.processed_jobs_tracker.mark_failed(transaction.id as i64).await?;
                return Err(e);
            }
        }

        Ok(())
    }
} 