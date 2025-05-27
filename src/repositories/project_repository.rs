use sqlx::PgPool;
use uuid::Uuid;
use crate::models::project_model::{Project,CreateProjectDTO};


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
     pub async fn create(pool: &PgPool, dto: &CreateProjectDTO) -> Result<Project, sqlx::Error> {
        let new_id = Uuid::new_v4();

        let project_index = sqlx::query_scalar!("SELECT COALESCE(MAX(project_index), 0) + 1 FROM project")
            .fetch_one(pool)
            .await?;

        let inserted_project = sqlx::query_as!(
            Project,
            r#"
            INSERT INTO project (id, project_index, title, icon)
            VALUES ($1, $2, $3, $4)
            RETURNING id, project_index, title, icon
            "#,
            new_id,
            project_index,
            dto.title,
            dto.icon
        )
        .fetch_one(pool)
        .await?;

        Ok(inserted_project)
    }
}
