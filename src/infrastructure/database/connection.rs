use crate::error::AppResult;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::fs;
use tracing::info;

pub async fn create_pool(database_url: &str) -> AppResult<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    Ok(pool)
}

pub async fn run_migrations(pool: &PgPool) -> AppResult<()> {
    info!("Running database migrations...");
    
    let sql_content = fs::read_to_string("migrations/init.sql")?;
    
    let statements: Vec<&str> = sql_content
        .split(';')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty() && !s.starts_with("--"))
        .collect();
    
    for statement in statements {
        if !statement.trim().is_empty() {
            sqlx::query(statement).execute(pool).await?;
        }
    }
    
    info!("Database migrations completed successfully");
    Ok(())
}
