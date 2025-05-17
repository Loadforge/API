use crate::models::project::Project;
use crate::repositories::project::ProjectRepository;
use sqlx::PgPool;

pub struct ProjectService;

impl ProjectService {
    pub async fn get_all_projects(pool: &PgPool) -> Result<Vec<Project>, sqlx::Error> {
        ProjectRepository::find_all(pool).await
    }

   
}
