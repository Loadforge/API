use actix_web::{web, HttpResponse, Responder};
use crate::services::project::ProjectService;
use sqlx::PgPool;

pub async fn get_projects_handler(db_pool: web::Data<PgPool>) -> impl Responder {
    match ProjectService::get_all_projects(db_pool.get_ref()).await {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao buscar projetos"),
    }
}


