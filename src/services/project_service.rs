use crate::models::project_model::Project;
use crate::repositories::project_repository::ProjectRepository;
use sqlx::PgPool;

pub struct ProjectService;

impl ProjectService {
    pub async fn get_all_projects(pool: &PgPool) -> Result<Vec<Project>, sqlx::Error> {
        ProjectRepository::find_all(pool).await
    }

   
}
