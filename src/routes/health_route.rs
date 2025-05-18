use actix_web::web;
use crate::handlers::health_handler::health_check_handler;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/health")
            .route("", web::get().to(health_check_handler))
    );
}
