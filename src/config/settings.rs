use anyhow::{Context, Result};
use serde::Deserialize;
use std::env;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub blockchain: BlockchainConfig,
    pub server: ServerConfig,
    pub worker: WorkerConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    pub url: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BlockchainConfig {
    pub rpc_url: String,
    pub private_key: String,
    pub chain_id: u64,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WorkerConfig {
    pub poll_interval_seconds: u64,
    pub lookback_hours: i64,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            database: DatabaseConfig {
                url: env::var("DATABASE_URL")
                    .context("DATABASE_URL environment variable is required")?,
                max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "5".to_string())
                    .parse()
                    .context("DATABASE_MAX_CONNECTIONS must be a valid number")?,
            },
            redis: RedisConfig {
                url: env::var("REDIS_URL")
                    .context("REDIS_URL environment variable is required")?,
            },
            blockchain: BlockchainConfig {
                rpc_url: env::var("RPC_URL")
                    .context("RPC_URL environment variable is required")?,
                private_key: env::var("PRIVATE_KEY")
                    .context("PRIVATE_KEY environment variable is required")?,
                chain_id: env::var("CHAIN_ID")
                    .unwrap_or_else(|_| "1".to_string())
                    .parse()
                    .context("CHAIN_ID must be a valid number")?,
            },
            server: ServerConfig {
                host: env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
                port: env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "3000".to_string())
                    .parse()
                    .context("SERVER_PORT must be a valid number")?,
            },
            worker: WorkerConfig {
                poll_interval_seconds: env::var("WORKER_POLL_INTERVAL_SECONDS")
                    .unwrap_or_else(|_| "5".to_string())
                    .parse()
                    .context("WORKER_POLL_INTERVAL_SECONDS must be a valid number")?,
                lookback_hours: env::var("WORKER_LOOKBACK_HOURS")
                    .unwrap_or_else(|_| "1".to_string())
                    .parse()
                    .context("WORKER_LOOKBACK_HOURS must be a valid number")?,
            },
        })
    }
} 