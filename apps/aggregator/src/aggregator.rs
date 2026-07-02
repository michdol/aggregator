use chrono::{DateTime, Utc};
use log::error;
use shared_models::Message;
use shared_models::{postgresql::PostgresClient, redis_client::RedisClient};
use sqlx::Arguments;
use sqlx::postgres::PgArguments;
use std::collections::HashMap;

const INSERT_TELEMETRY: &str = "INSERT INTO vehicle_telemetry (vehicle_id, latitude, longitude, altitude, speed, timestamp) VALUES ($1, $2, $3, $4, $5, $6)";

pub struct Aggregator {
    redis: RedisClient,
    postgres: PostgresClient,
    update_timestamps: HashMap<u64, DateTime<Utc>>,
}

impl Aggregator {
    pub fn new(postgres: PostgresClient, redis: RedisClient) -> Self {
        Self {
            redis: redis,
            postgres: postgres,
            update_timestamps: HashMap::new(),
        }
    }
    pub async fn aggregate(&mut self, message: &Message) {
        self.aggregate_single_vehicle(message).await;
    }

    pub async fn aggregate_single_vehicle(&mut self, message: &Message) {
        match self.update_timestamps.get(&message.payload.id) {
            Some(timestamp) => {
                let duration = message.timestamp - timestamp;
                if duration.num_minutes() >= 1 {
                    self.update_recent_positions(message).await;
                    self.update_timestamps
                        .insert(message.payload.id, message.timestamp);
                }
            }
            None => {
                self.update_recent_positions(message).await;
                self.update_timestamps
                    .insert(message.payload.id, message.timestamp);
            }
        }
    }

    pub async fn insert_single_row(&mut self, message: &Message) {
        // Insert to Postgresql 1 by 1
        let mut args = PgArguments::default();
        args.add(1);
        args.add(message.payload.gps.latitude);
        args.add(message.payload.gps.longitude);
        args.add(message.payload.gps.altitude);
        args.add(message.payload.gps.speed);
        args.add(message.timestamp.timestamp());
        self.postgres.insert(INSERT_TELEMETRY, args).await.unwrap();
    }

    // Insert to Postgresql streaming
    pub async fn stream_to_postgres() {
        todo!()
    }

    pub async fn set_redis_example(&mut self, message: &Message) {
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

    async fn update_recent_positions(&mut self, message: &Message) {
        let _ = self.redis.set_vehicle_messages(message).await;
    }
    // Insert to Redis aggregated last 10 minutes of messages
    //      insert 10 messages in the span of 1 minute from last 10 minutes
}
