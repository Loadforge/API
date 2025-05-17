use sqlx::{PgPool, postgres::PgPoolOptions};
use dotenvy::dotenv;
use std::env;

pub async fn connect_db() -> PgPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL not defined in .env");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Error to connect to the database")
}