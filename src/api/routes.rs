use crate::{
    application::worker::polling_worker::PollingWorker,
    config::Config,
    domain::services::transaction_processor::{PostgresTransactionRepository, TransactionProcessorService},
    error::AppResult,
    infrastructure::{blockchain::client::BlockchainClient, database::repositories::processed_jobs_repo::ProcessedJobsTracker},
    shared::traits::AppService
};
use axum::{
    extract::State,
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use redis::Client;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::info;

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub redis_client: Client,
    pub blockchain_client: BlockchainClient,
}

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn status(_state: State<Arc<AppState>>) -> Json<serde_json::Value> {
    let response = serde_json::json!({
        "status": "healthy",
        "service": "rust-polling",
        "database": "connected",
        "redis": "connected",
        "blockchain": "simulated"
    });
    
    Json(response)
}

pub fn create_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/status", get(status))
        .with_state(state)
}

pub async fn start_server(
    config: Config,
    db_pool: PgPool,
    redis_client: Client,
    blockchain_client: BlockchainClient,
) -> AppResult<()> {
    let state = Arc::new(AppState {
        db_pool: db_pool.clone(),
        redis_client: redis_client.clone(),
        blockchain_client: blockchain_client.clone(),
    });

    let app = create_router(state.clone());
    
    info!("Starting web server on http://{}:{}", config.server.host, config.server.port);
    
    let transaction_repository = Arc::new(PostgresTransactionRepository::new(db_pool.clone()));
    let processed_jobs_tracker = Arc::new(ProcessedJobsTracker::new(db_pool.clone()));
    let blockchain_service = Arc::new(blockchain_client);
    
    let transaction_processor = Arc::new(TransactionProcessorService::new(
        processed_jobs_tracker,
        blockchain_service,
    ));
    
    let mut worker = PollingWorker::new(
        config.worker,
        transaction_repository,
        transaction_processor,
    );
    
    tokio::spawn(async move {
        if let Err(e) = worker.start().await {
            tracing::error!("Worker error: {}", e);
        }
    });

    let listener = tokio::net::TcpListener::bind(format!("{}:{}", config.server.host, config.server.port)).await?;
    axum::serve(listener, app).await?;

    Ok(())
} 