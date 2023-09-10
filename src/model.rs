use ballista::prelude::{BallistaConfig, BallistaContext};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Clone, ToSchema)]
pub struct QueryJobRequest {
    #[schema(example = "SELECT * FROM test")]
    pub query: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, ToSchema)]
pub struct QueryJob {
    pub id: Option<String>,
    pub query: String,
    pub completed: Option<bool>,
    #[schema(value_type = Option<String>)]
    pub createdAt: Option<DateTime<Utc>>,
}

pub struct AppState {
    pub ballista_context: Arc<Mutex<BallistaContext>>,
}

impl AppState {
    pub async fn init() -> AppState {
        let ballista_config_builder =
            BallistaConfig::builder().set("ballista.with_information_schema", "true");
        let ballista_config = ballista_config_builder
            .build()
            .expect("Failed to build BallistaConfig");
        let ballista_context = BallistaContext::remote("localhost", 50050, &ballista_config)
            .await
            .expect("ballista_context");

        AppState {
            ballista_context: Arc::new(Mutex::new(ballista_context)),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct QueryOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}
