use actix_web::{web, HttpResponse, Responder};
use crate::services::project_service::ProjectService;
use crate::models::project_model::CreateProjectDTO;
use sqlx::PgPool;

pub async fn get_projects_handler(db_pool: web::Data<PgPool>) -> impl Responder {
    match ProjectService::get_all_projects(db_pool.get_ref()).await {
        Ok(projects) => HttpResponse::Ok().json(projects),
        Err(_) => HttpResponse::InternalServerError().body("Erro ao buscar projetos"),
    }
}
pub async fn create_project_handler(
    db_pool: web::Data<PgPool>,
    project: web::Json<CreateProjectDTO>,
) -> impl Responder {
    match ProjectService::create_project(db_pool.get_ref(), &project.into_inner()).await {
        Ok(inserted_project) => HttpResponse::Created().json(inserted_project),
        Err(e) => {
            eprintln!("Database insert error: {}", e);
            HttpResponse::InternalServerError().body("Error creating project")
        }
    }
}


