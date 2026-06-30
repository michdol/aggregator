use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub mod rabbitmq;
pub mod redis_client;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub timestamp: DateTime<Utc>,
    pub sensor: Sensor,
    pub payload: Vehicle,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Sensor {
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vehicle {
    pub id: String,
    pub gps: GpsData,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GpsData {
    pub latitude: f64,
    pub longitude: f64,
    pub altitude: f64,
    pub speed: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Degrees(pub f64);

impl Degrees {
    pub fn to_radians(self) -> Radians {
        Radians(self.0.to_radians())
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Meters(pub f64);

#[derive(Debug, Clone, Copy)]
pub struct Radians(pub f64);

impl Radians {
    pub fn cos(self) -> f64 {
        self.0.cos()
    }

    pub fn sin(self) -> f64 {
        self.0.sin()
    }
}

pub const EARTH_RADIUS: Meters = Meters(6_371_000.0);
