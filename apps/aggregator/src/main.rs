use aggregator::Aggregator;
use futures_util::stream::StreamExt;
use lapin::options::BasicAckOptions;
use shared_models::{SensorData, rabbitmq::RabbitMq, redis_client::RedisClient};

mod aggregator;

#[tokio::main]
async fn main() {
    println!("entering");
    let redis_client = RedisClient::new(String::from("redis://redis-service:6379")).await;
    println!("got client");
    let mut agg = Aggregator::new(redis_client);
    let amqp_url = "amqp://rabbitmq-service:5672/%2f";
    let rabbit = match RabbitMq::new(amqp_url, "trucks").await {
        Ok(instance) => instance,
        Err(err) => {
            panic!("failed connecting to Rabbit{}", err);
        }
    };
    println!("got rabbit");

    if let Ok(mut consumer) = rabbit.get_consumer().await {
        println!("got consumer");
        while let Some(message) = consumer.next().await {
            if let Ok(message) = message {
                match serde_json::from_slice::<SensorData>(&message.data) {
                    Ok(model) => {
                        println!("Data {:?}", model);
                        agg.aggregate(&model).await;
                    }
                    Err(err) => eprintln!("Error {:?}", err),
                }
                if let Err(e) = message.ack(BasicAckOptions::default()).await {
                    eprintln!("Error acking {:?}", e);
                }
            }
        }
    }
}
