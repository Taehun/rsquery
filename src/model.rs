use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use utoipa::ToSchema;

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct QueryJob {
    pub id: Option<String>,
    #[schema(example = "SELECT * FROM test")]
    pub query: String,
    pub completed: Option<bool>,
    #[schema(value_type = Option<String>)]
    pub createdAt: Option<DateTime<Utc>>,
}

pub struct AppState {
    pub jobs_db: Arc<Mutex<Vec<QueryJob>>>,
}

impl AppState {
    pub fn init() -> AppState {
        AppState {
            jobs_db: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct QueryOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}
