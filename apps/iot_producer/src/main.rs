use chrono::Utc;
use dotenvy::dotenv;
use log;
use shared_models::{
    Degrees, EARTH_RADIUS, GpsData, Message, Meters, Radians, Sensor, Vehicle, rabbitmq::RabbitMq,
};
use std::{
    env, thread,
    time::{Duration, Instant},
};

#[tokio::main]
async fn main() {
    env_logger::init();
    log::info!("Starting producer ⚠️");
    dotenv().ok();
    let rate: u64 = env::var("RATE").expect("RATE must be set").parse().unwrap();
    let rabbitmq_url: String = env::var("RABBITMQ_URL").expect("RABBITMQ_URL must be set");
    let queue_name: String = env::var("QUEUE_NAME").expect("QUEUE_NAME must be set");
    metrics_exporter_prometheus::PrometheusBuilder::new()
        .with_http_listener(([127, 0, 0, 1], 9000))
        .install();
    let rabbit = match RabbitMq::new(&rabbitmq_url, &queue_name).await {
        Ok(instance) => instance,
        Err(err) => {
            panic!("failed connecting to rabbit {}", err);
        }
    };
    log::info!("Got RabbitMq...✅");
    let mut message: Message = Message {
        timestamp: Utc::now(),
        sensor: Sensor {
            id: String::from("00000000-b54f-4ac9-9cda-68fe41410ce2"),
        },
        payload: Vehicle {
            id: String::from("11111111-b54f-4ac9-9cda-68fe41410ce2"),
            gps: GpsData {
                latitude: 52.00882034091296,
                longitude: 17.0333332,
                altitude: 0.1,
                speed: 27.7777,
            },
        },
    };
    log::info!("Starting sending messages");
    let sleep_duration = Duration::from_millis(1000 / rate);
    loop {
        let start = Instant::now();
        let (new_lat, new_lon) = generate_new_coordinates(
            message.payload.gps.latitude,
            message.payload.gps.longitude,
            message.payload.gps.speed,
            3.0,
            Degrees(15.0),
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

fn calculate_distance(speed: f64, time: f64) -> Meters {
    Meters(speed * time)
}

fn calculate_new_latitude(latitude: f64, distance: Meters, bearing: Radians) -> f64 {
    let distance_f64 = distance.0;
    let earth_radius_f64 = EARTH_RADIUS.0;
    let distance_over_earth_radius = distance_f64 / earth_radius_f64;
    let term1 = latitude.sin() * distance_over_earth_radius.cos();
    let term2 = latitude.cos() * distance_over_earth_radius.sin() * bearing.cos();
    let inside_arcsin = (term1 + term2).clamp(-1.0, 1.0);
    // Calculations were in radians, convert back to degrees
    inside_arcsin.asin().to_degrees()
}

fn calculate_new_longitude(
    longitude: f64,
    distance: Meters,
    bearing: Radians,
    latitude1: f64,
    latitude2: f64,
) -> f64 {
    let distance_f64 = distance.0;
    let earth_radius_f64 = EARTH_RADIUS.0;
    let distance_over_earth_radius = distance_f64 / earth_radius_f64;
    let term1 = bearing.sin() * distance_over_earth_radius.sin() * latitude1.cos();
    let term2 = distance_over_earth_radius.cos() - latitude1.sin() * latitude2.sin();
    let atan2 = term1.atan2(term2);

    // Calculations were in radians, convert back to degrees
    (longitude + atan2).to_degrees()
}
fn generate_new_coordinates(
    lat: f64,
    lon: f64,
    speed: f64,
    time: f64,
    bearing: Degrees,
) -> (f64, f64) {
    let distance = calculate_distance(speed, time);

    let new_lat = calculate_new_latitude(lat.to_radians(), distance, bearing.to_radians());
    let new_lon = calculate_new_longitude(
        lon.to_radians(),
        distance,
        bearing.to_radians(),
        lat.to_radians(),
        new_lat.to_radians(),
    );

    (new_lat, new_lon)
}
