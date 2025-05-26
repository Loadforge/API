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
    },
    pub async fn create(pool: &PgPool, project: &Project) -> Result<Project, sqlx::Error> {
    let inserted_project = sqlx::query_as!(
        Project,
        r#"
        INSERT INTO project (id,project_index, title, icon)
        VALUES ($1, $2, $3, $4)
        RETURNING id, project_index, title, icon
        "#,
        project.id
        project.project_index,
        project.title,
        project.icon,
    )
    .fetch_one(pool)
    .await?;

    Ok(inserted_project)
}
}
