use serde::{Deserialize, Serialize};

pub mod rabbitmq;
pub mod redis_client;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SensorData {
    pub id: String,
    pub payload: MeteorologicalPayload,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MeteorologicalPayload {
    pub ambient_temp_c: f32,
    pub wind_speed_ms: f32,
}
