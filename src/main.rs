use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use actix_cors::Cors;
use serde::Serialize;
use sqlx::PgPool;

mod config;
mod api; 
pub mod handlers;
pub mod services;
pub mod models;
pub mod repositories;



use config::db;
use api::project; 

#[derive(Serialize)]
struct HealthResponse {
    API_status: String,
    DB_status: String,
}

async fn health(db: web::Data<PgPool>) -> impl Responder {
    let result = sqlx::query("SELECT 1").execute(db.get_ref()).await;

    match result {
        Ok(_) => HttpResponse::Ok().json(HealthResponse {
            API_status: "ok".to_string(),
            DB_status: "ok".to_string(),
        }),
        Err(_) => HttpResponse::InternalServerError().json(HealthResponse {
            API_status: "ok".to_string(),
            DB_status: "database error".to_string(),
        }),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_pool = db::connect_db().await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db_pool.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .route("/health", web::get().to(health))
            .configure(project::config) 
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
