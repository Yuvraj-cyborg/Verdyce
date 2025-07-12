use redis::aio::Connection;
use redis::{AsyncCommands, Client, RedisResult};
use std::env;
use serde::{Serialize, de::DeserializeOwned};
use dotenvy;

pub async fn get_conn() -> RedisResult<Connection> {
    dotenvy::dotenv().ok(); 
    let url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    println!("Connecting to Redis at: {}", url);  // Debug print
    let client = Client::open(url)?;
    client.get_async_connection().await
}

pub async fn save_json<T: Serialize>(key: &str, value: &T) -> RedisResult<()> {
    let json = serde_json::to_string(value).map_err(|e| {
        redis::RedisError::from((redis::ErrorKind::TypeError, "serde_json error", format!("{:?}", e)))
    })?;
    let mut conn = get_conn().await?;
    conn.set(key, json).await
}

pub async fn get_json<T: DeserializeOwned>(key: &str) -> RedisResult<Option<T>> {
    let mut conn = get_conn().await?;
    let json: Option<String> = conn.get(key).await?;
    match json {
        Some(s) => {
            let deserialized = serde_json::from_str(&s).map_err(|e| {
                redis::RedisError::from((redis::ErrorKind::TypeError, "serde_json error", format!("{:?}", e)))
            })?;
            Ok(Some(deserialized))
        }
        None => Ok(None),
    }
}
