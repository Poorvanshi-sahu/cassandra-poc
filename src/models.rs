use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Option<Uuid>,
    pub name: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct PartialUser {
    pub name: Option<String>,
    pub email: Option<String>,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub message: String,
    pub success: bool,
    pub data: Option<T>,
}