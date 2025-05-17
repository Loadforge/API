use sqlx::{PgPool, postgres::PgPoolOptions};
use dotenv::dotenv;
use std::env;

pub async fn connect_db() -> PgPool {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL deve estar definida no .env");

    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Erro ao conectar ao banco")
}