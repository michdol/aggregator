use aggregator::Aggregator;
use futures_util::stream::StreamExt;
use lapin::options::BasicAckOptions;
use shared_models::{MeteorologicalPayload, RabbitMq, SensorData};

mod aggregator;

#[tokio::main]
async fn main() {
    let amqp_url = "amqp://rabbitmq-service:5672/%2f";
    let rabbit = match RabbitMq::new(amqp_url, "weather_telemetry").await {
        Ok(instance) => instance,
        Err(err) => {
            panic!("failed connecting to Rabbit{}", err);
        }
    };

    if let Ok(mut consumer) = rabbit.get_consumer().await {
        while let Some(message) = consumer.next().await {
            if let Ok(message) = message {
                match serde_json::from_slice::<SensorData>(&message.data) {
                    Ok(model) => {
                        println!("Data {:?}", model);
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
