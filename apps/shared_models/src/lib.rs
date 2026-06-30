use lapin::{
    BasicProperties, Channel, Connection, ConnectionProperties, Consumer, Result as LapinResult,
    options::*, types::FieldTable,
};
use serde::{Deserialize, Serialize};

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

#[derive(Debug)]
pub struct RabbitMq {
    channel: Channel,
    pub queue_name: String,
}

impl RabbitMq {
    pub async fn new(amqp_url: &str, queue_name: &str) -> Result<Self, lapin::Error> {
        let connection = Connection::connect(amqp_url, ConnectionProperties::default())
            .await
            .expect("Failed to connect to rabbit {}");

        let channel = connection
            .create_channel()
            .await
            .expect("Failed creating a channel");

        channel
            .queue_declare(
                queue_name.into(),
                QueueDeclareOptions::default(),
                Default::default(),
            )
            .await
            .expect("Failed declaring queue");

        Ok(Self {
            channel: channel,
            queue_name: queue_name.to_string(),
        })
    }

    pub async fn publish(&self, payload: &SensorData) {
        let msg = serde_json::to_string(payload).unwrap();
        self.channel
            .basic_publish(
                "".into(),
                self.queue_name.as_str().into(),
                BasicPublishOptions::default(),
                msg.as_bytes(),
                BasicProperties::default(),
            )
            .await
            .expect("failed sending data");
    }

    pub async fn get_consumer(&self) -> LapinResult<Consumer> {
        self.channel
            .basic_consume(
                self.queue_name.as_str().into(),
                "".into(),
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
    }
}
