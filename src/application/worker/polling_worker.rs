use crate::{
    config::WorkerConfig,
    error::AppResult,
    shared::traits::{AppService, TransactionProcessor, TransactionRepository},
};
use std::{sync::Arc, time::Duration};
use tracing::{error, info};

/// A worker that polls the database for new transactions
pub struct PollingWorker {
    config: WorkerConfig,
    transaction_repository: Arc<dyn TransactionRepository + Send + Sync>,
    transaction_processor: Arc<dyn TransactionProcessor + Send + Sync>,
    last_checked: chrono::DateTime<chrono::Utc>,
    running: bool,
}

impl PollingWorker {
    /// Creates a new polling worker
    pub fn new(
        config: WorkerConfig,
        transaction_repository: Arc<dyn TransactionRepository + Send + Sync>,
        transaction_processor: Arc<dyn TransactionProcessor + Send + Sync>,
    ) -> Self {
        let lookback_duration = chrono::Duration::hours(config.lookback_hours);
        Self {
            config,
            transaction_repository,
            transaction_processor,
            last_checked: chrono::Utc::now() - lookback_duration,
            running: false,
        }
    }

    /// Polls the database for new transactions
    async fn poll_once(&mut self) -> AppResult<()> {
        info!("Polling for new records...");

        let transactions = self
            .transaction_repository
            .fetch_new_transactions(self.last_checked)
            .await?;

        for transaction in transactions {
            if let Err(e) = self.transaction_processor.process_transaction(&transaction).await {
                error!("Error processing transaction {}: {}", transaction.id, e);
            }
        }

        self.last_checked = chrono::Utc::now();
        Ok(())
    }
}

#[async_trait::async_trait]
impl AppService for PollingWorker {
    async fn start(&mut self) -> AppResult<()> {
        info!("Starting polling worker...");
        self.running = true;

        while self.running {
            if let Err(e) = self.poll_once().await {
                error!("Error during polling: {}", e);
            }

            tokio::time::sleep(Duration::from_secs(self.config.poll_interval_seconds)).await;
        }

        Ok(())
    }

    async fn stop(&self) -> AppResult<()> {
        info!("Stopping polling worker...");
        // Note: In a real implementation, we'd need to handle graceful shutdown
        // For now, we'll rely on the running flag being set to false externally
        Ok(())
    }
}
