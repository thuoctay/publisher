use anyhow::Result;
use dotenvy::dotenv;
use rust_polling::{
    config::Config,
    create_pool, create_redis_client, run_migrations, BlockchainClient, start_server,
};
use tracing::info;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt::init();

    let config = Config::from_env()?;

    let db_pool = create_pool(&config.database.url).await?;
    let redis_client = create_redis_client(&config.redis.url)?;
    let blockchain_client = BlockchainClient::new(&config.blockchain.rpc_url, &config.blockchain.private_key)?;

    run_migrations(&db_pool).await?;

    info!("Starting Polling Service with Axum web server...");

    start_server(config, db_pool, redis_client, blockchain_client).await?;

    Ok(())
}
