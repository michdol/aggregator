use crate::Message;
use log;
use redis::AsyncCommands;

#[derive(Clone)]
pub struct RedisClient {
    connection: redis::aio::MultiplexedConnection,
}

impl RedisClient {
    pub async fn new(url: String) -> Self {
        let client = match redis::Client::open(url.clone()) {
            Ok(c) => c,
            Err(e) => {
                panic!("invalid redis url, {:?}", e);
            }
        };
        let connection = match client.get_multiplexed_tokio_connection().await {
            Ok(conn) => conn,
            Err(err) => {
                panic!("failed connecting to redis {:?}", err);
            }
        };
        Self {
            connection: connection,
        }
    }

    pub async fn set(&self, key: u64, value: String) -> redis::RedisResult<()> {
        let mut con = self.connection.clone();
        tokio::spawn(async move {
            let _: Result<(), redis::RedisError> = con.set(key, value).await;
        });
        Ok(())
    }

    pub async fn set_vehicle_messages(&mut self, message: &Message) -> redis::RedisResult<()> {
        let redis_key: String = format!("messages:{}", message.payload.id);
        let json_string: String = serde_json::to_string(message).unwrap();

        let mut con = self.connection.clone();
        let mut result: redis::RedisResult<usize> = con.rpush(&redis_key, json_string).await;
        match result {
            Ok(new_length) => {
                if new_length > 10 {
                    let status: redis::RedisResult<String> = con.ltrim(&redis_key, -10, -1).await;
                    match result {
                        Ok(_) => {}
                        Err(err) => log::error!(
                            "Failed trimming messages for key: {}\nerr: {}",
                            redis_key,
                            err
                        ),
                    }
                }
            }
            Err(err) => log::error!("Failed to update key: {}\nerr: {}", redis_key, err),
        }

        Ok(())
    }
}
