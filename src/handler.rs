use crate::{
    model::{AppState, QueryJob},
    response::{Field, GenericResponse, QueryJobResponse, QueryJobResult, Schema},
};
use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use uuid::Uuid;
use std::{thread, time};

#[get("/health")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "RSQuery REST API server is running!";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string(),
    };
    HttpResponse::Ok().json(response_json)
}

/// Create new query job to shared in-memory storage.
///
/// Post a new `QueryJob` in request body as json to store it. Api will return created `QueryJob` on success
///
/// One could call the api with.
/// ```text
/// curl -X POST http://localhost:8080/api/query -H "Content-Type: application/json" -d '{"query": "SELECT * FROM test"}'
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

    let mut job = body.to_owned();

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

    job.completed = Some(true);
    println!("{:?}", job);

    let json_response = QueryJobResponse {
        status: "success".to_string(),
        result: query_result,
    };
    thread::sleep(time::Duration::from_millis(300)); // Delay for progress bar test

    HttpResponse::Ok().json(json_response)
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(create_query_job_handler);

    conf.service(scope);
}
