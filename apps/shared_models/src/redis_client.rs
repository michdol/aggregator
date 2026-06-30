use redis::AsyncCommands;

#[derive(Clone)]
pub struct RedisClient {
    connection: redis::aio::MultiplexedConnection,
}

impl RedisClient {
    pub async fn new(url: String) -> Self {
        let client = match redis::Client::open(url.clone()) {
            Ok(c) => c,
            Err(e) => {
                panic!("invalid redis url, {:?}", e);
            }
        };
        let connection = match client.get_multiplexed_tokio_connection().await {
            Ok(conn) => conn,
            Err(err) => {
                panic!("failed connecting to redis {:?}", err);
            }
        };
        Self {
            connection: connection,
        }
    }

    pub async fn set(&self, key: String, value: String) -> redis::RedisResult<()> {
        let mut con = self.connection.clone();
        println!("setting");
        tokio::spawn(async move {
            println!("setting inside tokio thread");
            let _: Result<(), redis::RedisError> = con.set(key, value).await;
        });
        Ok(())
    }
}
