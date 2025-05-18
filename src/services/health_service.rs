use sqlx::PgPool;
use crate::repositories::health_repository::HealthRepository;

pub struct HealthService;

impl HealthService {
    pub async fn check_health(pool: &PgPool) -> bool {
        HealthRepository::check_database(pool).await
    }
}
