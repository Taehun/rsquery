use crate::model::QueryJob;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct QueryJobData {
    pub job: QueryJob,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct QueryJobResult {
    pub total_rows: u32,
    pub columns: Vec<Vec<String>>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct QueryJobResponse {
    pub status: String,
    pub result: QueryJobResult,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct QueryJobResponseString {
    pub status: String,
    pub result: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub(super) enum ErrorResponse {
    /// When Todo is not found by search term.
    NotFound(String),
    InternalServerError(String),
    /// When todo endpoint was called without correct credentials
    Unauthorized(String),
}
