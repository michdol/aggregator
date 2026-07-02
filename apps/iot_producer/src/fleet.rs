use chrono::Utc;
use shared_models::{Degrees, EARTH_RADIUS, GpsData, Message, Meters, Radians, Sensor, Vehicle};
use std::iter::Iterator;

pub struct FleetIterator {
    current_index: i64,
    messages: Vec<Message>,
    count: usize,
}

impl FleetIterator {
    pub fn new(messages: Vec<Message>) -> Self {
        let count = messages.len();
        Self {
            current_index: -1,
            messages: messages,
            count: count,
        }
    }
}

impl Iterator for FleetIterator {
    type Item = Message;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_index += 1;
        if self.current_index as usize >= self.count {
            self.current_index = 0;
        }
        Some(self.messages[self.current_index as usize].clone())
    }
}

pub fn create_fleet(fleet_count: u64) -> Vec<Message> {
    let mut messages = Vec::new();
    for i in 0..fleet_count {
        let sensor_id = format!("{:0>8}-b54f-4ac9-9cda-68fe41410ce2", i);
        let message: Message = Message {
            timestamp: Utc::now(),
            sensor: Sensor {
                id: String::from(sensor_id),
            },
            payload: Vehicle {
                id: i + 1,
                gps: GpsData {
                    latitude: 52.00882034091296,
                    longitude: 17.0333332,
                    altitude: 0.1,
                    speed: 27.7777,
                },
            },
        };
        messages.push(message);
    }
    messages
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
pub fn generate_new_coordinates(
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_fleet() {
        let fleet_count = 2;
        let messages = create_fleet(fleet_count);
        assert_eq!(messages.len(), 2);
        assert!(
            messages[0].sensor.id.starts_with("00000000"),
            "{}",
            messages[0].sensor.id
        );
        assert!(
            messages[1].sensor.id.starts_with("00000001"),
            "{}",
            messages[1].sensor.id
        );
        assert_eq!(messages[0].payload.id, 1);
        assert_eq!(messages[1].payload.id, 2);
    }

    #[test]
    fn test_fleet_iterator() {
        let messages = create_fleet(2);
        let mut iterator = FleetIterator::new(messages);
        assert_eq!(iterator.current_index, -1);
        assert_eq!(iterator.count, 2);

        // Assert iterator loops through messages
        let message: Message = iterator.next().unwrap();
        assert_eq!(message.payload.id, 1);
        let message: Message = iterator.next().unwrap();
        assert_eq!(message.payload.id, 2);
        let message: Message = iterator.next().unwrap();
        assert_eq!(message.payload.id, 1);
        let message: Message = iterator.next().unwrap();
        assert_eq!(message.payload.id, 2);
        let message: Message = iterator.next().unwrap();
        assert_eq!(message.payload.id, 1);
    }
}
