use shared_models::SensorData;
use shared_models::redis_client::RedisClient;
use std::collections::HashMap;
pub struct Aggregator {
    sensors: HashMap<String, HashMap<String, f32>>,
    redis: RedisClient,
}

impl Aggregator {
    pub fn new(redis: RedisClient) -> Self {
        Self {
            sensors: HashMap::new(),
            redis: redis,
        }
    }
    pub async fn aggregate(&mut self, data: &SensorData) {
        println!("{}", data.payload.wind_speed_ms);
        let mut hash: HashMap<String, f32> = HashMap::new();
        hash.insert(String::from("wind_speed"), data.payload.wind_speed_ms);
        self.sensors.insert(data.id.clone(), hash);
        println!("calling redis client");
        match self
            .redis
            .set(data.id.clone(), data.payload.wind_speed_ms.to_string())
            .await
        {
            Ok(_) => (),
            Err(err) => panic!("fuck this should be handled {:?}", err),
        }
    }
}
