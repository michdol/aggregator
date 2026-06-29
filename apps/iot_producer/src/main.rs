/*
* k port-forward service/rabbitmq-service 15672:15672
*
*/

use rand::thread_rng;
use rand_distr::{Distribution, Normal};
use shared_models::{MeteorologicalPayload, RabbitMq, SensorData};

// use lapin::{BasicProperties, Connection, ConnectionProperties, options::*};

#[tokio::main]
async fn main() {
    let amqp_url = "amqp://rabbitmq-service:5672/%2f";
    let queue_name = "weather_telemetry";
    let rabbit = match RabbitMq::new(amqp_url, queue_name).await {
        Ok(instance) => instance,
        Err(err) => {
            panic!("failed connecting to rabbit {}", err);
        }
    };
    let test_temp = 15.0;
    let test_wind = 2.0;
    let mut data: SensorData = SensorData {
        id: String::from("test_id_1"),
        payload: MeteorologicalPayload {
            ambient_temp_c: 0.0,
            wind_speed_ms: 0.0,
        },
    };
    loop {
        data.payload.ambient_temp_c = generate_value_with_noise(test_temp, 0.3);
        data.payload.wind_speed_ms = generate_value_with_noise(test_wind, 0.3);
        // println!("{:?}", data);

        rabbit.publish(&data).await
    }
}

fn generate_value_with_noise(baseline: f32, standard_deviation: f32) -> f32 {
    let mut rng = thread_rng();

    let normal = Normal::new(0.0, standard_deviation).unwrap();
    let deviation = normal.sample(&mut rng);

    baseline + deviation
}
/*
#[tokio::main]
async fn main() {
    let amqp_url = "amqp://rabbitmq-service:5672/%2f";
    let connection = Connection::connect(amqp_url, ConnectionProperties::default())
        .await
        .expect("Failed connecting to rabbit");

    let channel = connection
        .create_channel()
        .await
        .expect("Failed creating channel");

    channel
        .queue_declare(
            "weather_telemetry",
            QueueDeclareOptions::default(),
            Default::default(),
        )
        .await
        .expect("Failed creating queue");

    let test_temp = 15.0;
    let test_wind = 2.0;
    let mut data: SensorData = SensorData {
        id: String::from("test_id_1"),
        payload: MeteorologicalPayload {
            ambient_temp_c: 0.0,
            wind_speed_ms: 0.0,
        },
    };
    loop {
        data.payload.ambient_temp_c = generate_value_with_noise(test_temp, 0.3);
        data.payload.wind_speed_ms = generate_value_with_noise(test_wind, 0.3);
        // println!("{:?}", data);

        let msg = serde_json::to_string(&data).unwrap();
        channel
            .basic_publish(
                "".into(),
                "weather_telemetry".into(),
                BasicPublishOptions::default(),
                msg.as_bytes(),
                BasicProperties::default(),
            )
            .await
            .expect("Failed to send data");
    }
}


*/
