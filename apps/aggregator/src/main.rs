use aggregator::Aggregator;
use dotenvy::dotenv;
use futures_util::stream::StreamExt;
use lapin::options::BasicAckOptions;
use log::{error, info};
use shared_models::{
    Message, postgresql::PostgresClient, rabbitmq::RabbitMq, redis_client::RedisClient,
};
use sqlx::PgPool;
use std::{env, time::Instant};

mod aggregator;

#[tokio::main]
async fn main() {
    env_logger::init();
    info!("Starting aggregator...⚠️");
    dotenv().ok();
    let redis_url: String = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let rabbitmq_url: String = env::var("RABBITMQ_URL").expect("RABBITMQ_URL must be set");
    let postgres_url: String = env::var("POSTGRES_URL").expect("POSTGRES_URL must be set");
    let _ = metrics_exporter_prometheus::PrometheusBuilder::new()
        .with_http_listener(([127, 0, 0, 1], 9001))
        .install();

    let pool = PgPool::connect(&postgres_url).await.unwrap();
    let postgres_client = PostgresClient::new(pool);
    let redis_client = RedisClient::new(String::from(redis_url)).await;
    info!("Redis client up and running...✅");
    let mut agg = Aggregator::new(postgres_client, redis_client);

    let rabbit = match RabbitMq::new(&rabbitmq_url, "trucks").await {
        Ok(instance) => instance,
        Err(err) => {
            panic!("failed connecting to Rabbit{}", err);
        }
    };
    info!("Got RabbitMq...✅");

    info!("Starting consumer...🚧");
    if let Ok(mut consumer) = rabbit.get_consumer().await {
        info!("Listening for messages from consumer...🚧");
        while let Some(message) = consumer.next().await {
            let start = Instant::now();
            if let Ok(message) = message {
                match serde_json::from_slice::<Message>(&message.data) {
                    Ok(model) => {
                        agg.aggregate(&model).await;
                    }
                    Err(err) => error!("⚠️ Error serializing message: {:?}", err),
                }
                if let Err(e) = message.ack(BasicAckOptions::default()).await {
                    error!("⚠️ Error acking message: {:?}", e)
                }
            }
            metrics::histogram!("aggregator_aggregation_duration")
                .record(start.elapsed().as_secs_f64());
        }
    }
    info!("Exiting...🎉");
}
