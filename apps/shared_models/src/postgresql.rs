use sqlx::PgPool;

pub struct PostgresClient {
    pool: PgPool,
}

impl PostgresClient {
    pub fn new(pool: PgPool) -> Self {
        Self { pool: pool }
    }

    pub async fn insert(
        &self,
        query: &str,
        args: sqlx::postgres::PgArguments,
    ) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        sqlx::query_with(query, args).execute(&self.pool).await
    }
}
