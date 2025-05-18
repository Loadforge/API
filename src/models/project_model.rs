use serde::Serialize;
use uuid::Uuid;

#[derive(Serialize)]
pub struct Project {
    pub id: Uuid,
    pub project_index: i32,
    pub title: String,
    pub icon: String,
}