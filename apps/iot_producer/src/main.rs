use chrono::Utc;
use shared_models::{
    Degrees, EARTH_RADIUS, GpsData, Message, Meters, Radians, Sensor, Vehicle, rabbitmq::RabbitMq,
};

#[tokio::main]
async fn main() {
    let amqp_url = "amqp://rabbitmq-service:5672/%2f";
    let queue_name = "trucks";
    let rabbit = match RabbitMq::new(amqp_url, queue_name).await {
        Ok(instance) => instance,
        Err(err) => {
            panic!("failed connecting to rabbit {}", err);
        }
    };
    let mut message: Message = Message {
        timestamp: Utc::now(),
        sensor: Sensor {
            id: String::from("test_id"),
        },
        payload: Vehicle {
            id: String::from("vehicle_id"),
            gps: GpsData {
                latitude: 52.00882034091296,
                longitude: 17.0333332,
                altitude: 0.1,
                speed: 27.7777,
            },
        },
    };
    loop {
        let (new_lat, new_lon) = generate_new_coordinates(
            message.payload.gps.latitude,
            message.payload.gps.longitude,
            message.payload.gps.speed,
            3.0,
            Degrees(15.0),
        );
        message.payload.gps.latitude = new_lat;
        message.payload.gps.longitude = new_lon;
        rabbit.publish(&message).await
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
