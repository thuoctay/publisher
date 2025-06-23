pub mod api;
pub mod application;
pub mod config;
pub mod domain;
pub mod error;
pub mod infrastructure;
pub mod shared;

pub use api::routes::{start_server, AppState};
pub use application::worker::polling_worker::PollingWorker;
pub use config::Config;
pub use domain::models::transaction::{Transaction, TransactionPayload};
pub use domain::services::transaction_processor::{PostgresTransactionRepository, TransactionProcessorService};
pub use error::{AppError, AppResult};
pub use infrastructure::database::connection::{create_pool, run_migrations};
pub use infrastructure::database::repositories::processed_jobs_repo::ProcessedJobsTracker;
pub use infrastructure::blockchain::client::BlockchainClient;
pub use infrastructure::redis::client::create_redis_client;
pub use shared::traits::*;
