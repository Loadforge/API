use sqlx::PgPool;

pub struct HealthRepository;

impl HealthRepository {
    pub async fn check_database(pool: &PgPool) -> bool {
        sqlx::query("SELECT 1")
            .fetch_one(pool)
            .await
            .is_ok()
    }
}
