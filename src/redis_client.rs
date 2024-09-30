use std::sync::{Arc, Mutex};

use redis::{Client, Commands, RedisResult};

pub struct RedisClient {
    pub connection: Arc<Mutex<Client>>,
}

impl RedisClient {
    pub async fn new(redis_url: &str) -> RedisResult<Self> {
        let client = Client::open(redis_url)?;
        Ok(Self {
            connection: Arc::new(Mutex::new(client)),
        })
    }

    pub async fn set(&self, key: &str, value: &str) -> RedisResult<()> {
        let conn = self.connection.lock().unwrap();
        conn.get_connection().unwrap().set(key, value)
    }

    pub async fn get(&self, key: &str) -> RedisResult<String> {
        let conn = self.connection.lock().unwrap();
        conn.get_connection()?.get(key)
    }
}

#[cfg(test)]
mod tests {
    use crate::redis_client::RedisClient;
    use std::env;

    #[tokio::test]
    async fn test_redis_connection() {
        dotenv::dotenv().ok();
        let redis_url = env::var("REDISCLOUD_URL").expect("REDISCLOUD_URL must be set");
        let client = RedisClient::new(&redis_url).await;
        assert!(client.is_ok(), "Failed to connect to Redis");
    }
}
