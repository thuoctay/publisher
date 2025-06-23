use crate::{
    error::AppResult,
    shared::traits::ProcessedJobsTracker as ProcessedJobsTrackerTrait,
};
use async_trait::async_trait;
use sqlx::{PgPool, Row};
use tracing::{debug, error, info};

pub struct ProcessedJobsTracker {
    pool: PgPool,
}

impl ProcessedJobsTracker {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_all_processed(&self) -> AppResult<Vec<(i64, Option<String>, String)>> {
        let rows = sqlx::query(
            "SELECT record_id, tx_hash, status FROM processed_jobs ORDER BY updated_at DESC"
        )
        .fetch_all(&self.pool)
        .await?;

        let processed_jobs = rows
            .into_iter()
            .map(|row| {
                let record_id: i64 = row.get("record_id");
                let tx_hash: Option<String> = row.get("tx_hash");
                let status: String = row.get("status");
                (record_id, tx_hash, status)
            })
            .collect();

        Ok(processed_jobs)
    }
}

#[async_trait]
impl ProcessedJobsTrackerTrait for ProcessedJobsTracker {
    async fn is_processed(&self, record_id: i64) -> AppResult<bool> {
        let row = sqlx::query("SELECT record_id FROM processed_jobs WHERE record_id = $1")
            .bind(record_id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(row.is_some())
    }

    async fn mark_pending(&self, record_id: i64) -> AppResult<()> {
        let result = sqlx::query(
            "INSERT INTO processed_jobs (record_id, status) VALUES ($1, 'pending') ON CONFLICT (record_id) DO NOTHING"
        )
        .bind(record_id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() > 0 {
            debug!("Marked record {} as pending", record_id);
        } else {
            debug!("Record {} was already marked as pending", record_id);
        }

        Ok(())
    }

    async fn mark_sent(&self, record_id: i64, tx_hash: &str) -> AppResult<()> {
        let result = sqlx::query(
            "UPDATE processed_jobs SET tx_hash = $1, status = 'sent', updated_at = CURRENT_TIMESTAMP WHERE record_id = $2"
        )
        .bind(tx_hash)
        .bind(record_id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() > 0 {
            info!("Marked record {} as sent with tx_hash: {}", record_id, tx_hash);
        } else {
            error!("Failed to update record {} as sent", record_id);
        }

        Ok(())
    }

    async fn mark_failed(&self, record_id: i64) -> AppResult<()> {
        let result = sqlx::query(
            "UPDATE processed_jobs SET status = 'failed', updated_at = CURRENT_TIMESTAMP WHERE record_id = $1"
        )
        .bind(record_id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() > 0 {
            error!("Marked record {} as failed", record_id);
        }

        Ok(())
    }
} 