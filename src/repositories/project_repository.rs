use sqlx::PgPool;
use crate::models::project_model::Project;

pub struct ProjectRepository;

impl ProjectRepository {
    pub async fn find_all(pool: &PgPool) -> Result<Vec<Project>, sqlx::Error> {
        let projects = sqlx::query_as!(
            Project,
            r#"
            SELECT id, project_index, title, icon
            FROM project
            "#
        )
        .fetch_all(pool)
        .await?;

        Ok(projects)
    }
}
