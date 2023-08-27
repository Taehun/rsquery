use crate::{
    model::{AppState, QueryJob},
    response::{Field, GenericResponse, QueryJobResponse, QueryJobResult, Schema},
};
use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use uuid::Uuid;
use std::{thread, time};

/// Check the server health
///
///
/// One could call the api with.
/// ```text
/// curl -X GET -H "Content-Type: application/json" https://rsquery-api.taehun.dev/health
/// ```
#[utoipa::path(
    path = "/health",
    responses(
        (status = 200, description = "Query job is created successfully", body = GenericResponse),
    )
)]
#[get("/health")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "RSQuery REST API server is running!";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}

/// Run a SQL query and response it's result
///
/// Pass the SQL query as a POST request parameter, and respond with the results of the execution.
///
/// One could call the api with.
/// ```text
/// curl -X POST -H "Content-Type: application/json" -d '{"query": "SELECT * FROM test"}' https://rsquery-api.taehun.dev/api/query
/// ```
#[utoipa::path(
    path = "/api/query",
    request_body = QueryJob,
    responses(
        (status = 200, description = "Query job is created successfully", body = QueryJobResponse),
    )
)]
#[post("/query")]
async fn create_query_job_handler(
    mut body: web::Json<QueryJob>,
    data: web::Data<AppState>,
) -> impl Responder {
    let uuid_id = Uuid::new_v4();
    let datetime = Utc::now();

    body.id = Some(uuid_id.to_string());
    body.completed = Some(false);
    body.createdAt = Some(datetime);

    let mut req_data = body.to_owned();

    // TODO: Query to Ballista...
    // Sample resonse data
    let query_result: QueryJobResult = QueryJobResult {
        schema: Schema {
            fields: vec![
                Field {
                    name: "fullVisitorId".to_string(),
                    field_type: "STRING".to_string(),
                    mode: "NULLABLE".to_string(),
                },
                Field {
                    name: "visitStartTime".to_string(),
                    field_type: "INTEGER".to_string(),
                    mode: "NULLABLE".to_string(),
                },
                Field {
                    name: "date".to_string(),
                    field_type: "TIMESTAMP".to_string(),
                    mode: "NULLABLE".to_string(),
                },
                Field {
                    name: "deviceCategory".to_string(),
                    field_type: "STRING".to_string(),
                    mode: "NULLABLE".to_string(),
                },
                Field {
                    name: "isMobile".to_string(),
                    field_type: "BOOLEAN".to_string(),
                    mode: "NULLABLE".to_string(),
                },
            ],
        },
        total_rows: 3,
        rows: vec![
            vec![
                "0550235018201479682".to_string(),
                "1471527222".to_string(),
                "1.4714784E9".to_string(),
                "mobile".to_string(),
                "true".to_string(),
            ],
            vec![
                "0550235018201479682".to_string(),
                "1471527222".to_string(),
                "1.4714784E9".to_string(),
                "mobile".to_string(),
                "true".to_string(),
            ],
            vec![
                "0550235018201479682".to_string(),
                "1471527222".to_string(),
                "1.4714784E9".to_string(),
                "mobile".to_string(),
                "true".to_string(),
            ],
        ],
    };

    println!("{}", req_data.query);

    let json_response = QueryJobResponse {
        status: "success".to_string(),
        result: query_result,
    };
    thread::sleep(time::Duration::from_millis(300)); // Delay for progress bar test

    HttpResponse::Ok().json(json_response)
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(create_query_job_handler);

    conf.service(health_checker_handler).service(scope);
}
