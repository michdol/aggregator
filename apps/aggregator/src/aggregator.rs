use log::error;
use shared_models::Message;
use shared_models::{postgresql::PostgresClient, redis_client::RedisClient};
use sqlx::Arguments;
use sqlx::postgres::PgArguments;

const INSERT_TELEMETRY: &str = "INSERT INTO vehicle_telemetry (vehicle_id, latitude, longitude, altitude, speed, timestamp) VALUES ($1, $2, $3, $4, $5, $6)";

pub struct Aggregator {
    redis: RedisClient,
    postgres: PostgresClient,
}

impl Aggregator {
    pub fn new(postgres: PostgresClient, redis: RedisClient) -> Self {
        Self {
            redis: redis,
            postgres: postgres,
        }
    }
    pub async fn aggregate(&mut self, message: &Message) {
        let mut args = PgArguments::default();
        args.add(1);
        args.add(message.payload.gps.latitude);
        args.add(message.payload.gps.longitude);
        args.add(message.payload.gps.altitude);
        args.add(message.payload.gps.speed);
        args.add(message.timestamp.timestamp());
        self.postgres.insert(INSERT_TELEMETRY, args).await.unwrap();
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
