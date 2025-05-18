use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::services::health_service::HealthService;
use crate::models::health_model::HealthStatus;

pub async fn health_check_handler(pool: web::Data<PgPool>) -> impl Responder {
    let db_ok = HealthService::check_health(pool.get_ref()).await;

    let status = HealthStatus {
        api_status: "up".to_string(),
        db_status: if db_ok { "up".to_string() } else { "down".to_string() },
    };

    HttpResponse::Ok().json(status)
}
