/*
* kubectl port-forward service/rabbitmq-service 15672:15672
* kubectl port-forward service/rabbitmq-service 5672:5672
*
*/

use rand::thread_rng;
use rand_distr::{Distribution, Normal};
use shared_models::{MeteorologicalPayload, SensorData, rabbitmq::RabbitMq};

// use lapin::{BasicProperties, Connection, ConnectionProperties, options::*};

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
