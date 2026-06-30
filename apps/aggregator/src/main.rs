use aggregator::Aggregator;
use futures_util::stream::StreamExt;
use lapin::options::BasicAckOptions;
use log::{error, info};
use shared_models::{Message, rabbitmq::RabbitMq, redis_client::RedisClient};

mod aggregator;

#[tokio::main]
async fn main() {
    info!("Starting aggregator...⚠️");
    let redis_client = RedisClient::new(String::from("redis://redis-service:6379")).await;
    info!("Redis client up and running...✅");
    let mut agg = Aggregator::new(redis_client);
    let amqp_url = "amqp://rabbitmq-service:5672/%2f";
    let rabbit = match RabbitMq::new(amqp_url, "trucks").await {
        Ok(instance) => instance,
        Err(err) => {
            panic!("failed connecting to Rabbit{}", err);
        }
    };
    info!("Got RabbitMq...✅");

    info!("Starting consumer...🚧");
    if let Ok(mut consumer) = rabbit.get_consumer().await {
        while let Some(message) = consumer.next().await {
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
        }
    }
    info!("Exiting...🎉");
}
