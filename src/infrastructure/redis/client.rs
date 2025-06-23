use crate::error::AppResult;
use redis::Client;

pub fn create_redis_client(redis_url: &str) -> AppResult<Client> {
    let client = Client::open(redis_url)?;
    Ok(client)
}
