use crate::model::QueryJob;
use serde::{Deserialize, Serialize};
use serde_json::Value;
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
pub struct Schema {
    pub fields: Vec<String>,
    pub types: Vec<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct QueryJobResult {
    pub total_rows: u32,
    pub schema: Schema,
    pub columns: Vec<Vec<Value>>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct QueryJobResponse {
    pub message: String,
    pub res_type: String,
    pub result: QueryJobResult,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub(super) enum ErrorResponse {
    /// When Todo is not found by search term.
    NotFound(String),
    InternalServerError(String),
    /// When todo endpoint was called without correct credentials
    Unauthorized(String),
}
