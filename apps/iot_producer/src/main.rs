use crate::fleet::{FleetIterator, create_fleet, generate_new_coordinates};
use chrono::Utc;
use dotenvy::dotenv;
use log;
use shared_models::{Degrees, rabbitmq::RabbitMq};
use std::{
    env, thread,
    time::{Duration, Instant},
};

mod fleet;

#[tokio::main]
async fn main() {
    env_logger::init();
    log::info!("Starting producer ⚠️");
    dotenv().ok();
    let rate: u64 = env::var("RATE").expect("RATE must be set").parse().unwrap();
    let fleet_count: u64 = env::var("FLEET_COUNT")
        .expect("FLEET_COUNT must be set")
        .parse()
        .unwrap();
    let rabbitmq_url: String = env::var("RABBITMQ_URL").expect("RABBITMQ_URL must be set");
    let queue_name: String = env::var("QUEUE_NAME").expect("QUEUE_NAME must be set");
    let _ = metrics_exporter_prometheus::PrometheusBuilder::new()
        .with_http_listener(([127, 0, 0, 1], 9000))
        .install();
    let rabbit = match RabbitMq::new(&rabbitmq_url, &queue_name).await {
        Ok(instance) => instance,
        Err(err) => {
            panic!("failed connecting to rabbit {}", err);
        }
    };
    log::info!("Got RabbitMq...✅");
    let messages = create_fleet(fleet_count);
    let mut iterator = FleetIterator::new(messages);
    log::info!("Starting sending messages");
    let sleep_duration = Duration::from_millis(1000 / rate);
    let time_interval = sleep_duration.as_secs_f64();
    let bearing = Degrees(0.05);
    loop {
        let start = Instant::now();
        let mut message = iterator.next().unwrap();
        let (new_lat, new_lon) = generate_new_coordinates(
            message.payload.gps.latitude,
            message.payload.gps.longitude,
            message.payload.gps.speed,
            time_interval,
            bearing,
        );
        message.payload.gps.latitude = new_lat;
        message.payload.gps.longitude = new_lon;
        message.timestamp = Utc::now();
        rabbit.publish(&message).await;
        thread::sleep(sleep_duration);
        let duration = start.elapsed();
        metrics::histogram!("producer_write_duration").record(duration.as_secs_f64());
    }
}
