use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::model::QueryJob;

#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct QueryJobData {
    pub job: QueryJob,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub struct QueryJobResponse {
    pub status: String,
    pub result: QueryJobData,
}

#[derive(Serialize, Deserialize, Clone, ToSchema)]
pub(super) enum ErrorResponse {
    /// When Todo is not found by search term.
    NotFound(String),
    /// When there is a conflict storing a new todo.
    Conflict(String),
    /// When todo endpoint was called without correct credentials
    Unauthorized(String),
}
