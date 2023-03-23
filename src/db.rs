use sqlx::{pool::PoolOptions, PgPool};

pub async fn create_pool(url: &String) -> PgPool {
    PoolOptions::new()
        .max_connections(10)
        .connect(url)
        .await
        .expect("COULD NOT CONNECT TO THE DB")
}
