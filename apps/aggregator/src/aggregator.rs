use shared_models::{MeteorologicalPayload, SensorData};
use std::collections::HashMap;

pub struct Aggregator {
    sensors: HashMap<String, HashMap<String, f32>>,
}

impl Aggregator {
    pub fn new() -> Self {
        Self {
            sensors: HashMap::new(),
        }
    }
    pub fn aggregate(&mut self, data: &SensorData) {
        println!("{}", data.payload.wind_speed_ms);
        let mut hash: HashMap<String, f32> = HashMap::new();
        hash.insert(String::from("wind_speed"), data.payload.wind_speed_ms);
        self.sensors.insert(data.id.clone(), hash);
    }
}
