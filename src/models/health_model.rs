use serde::Serialize;

#[derive(Serialize)]
pub struct HealthStatus {
    pub api_status: String,
    pub db_status: String,
}
