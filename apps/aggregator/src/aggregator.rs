use log::error;
use shared_models::Message;
use shared_models::redis_client::RedisClient;

pub struct Aggregator {
    redis: RedisClient,
}

impl Aggregator {
    pub fn new(redis: RedisClient) -> Self {
        Self { redis: redis }
    }
    pub async fn aggregate(&mut self, message: &Message) {
        match self
            .redis
            .set(
                message.payload.id.clone(),
                serde_json::to_string(message).unwrap(),
            )
            .await
        {
            Ok(_) => (),
            Err(err) => error!("⚠️ Error setting key {}: {:?}", message.payload.id, err),
        }
    }
}
