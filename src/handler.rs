use crate::{
    handle_result::record_batch_to_vec,
    model::{AppState, QueryJob},
    response::{ErrorResponse, GenericResponse, QueryJobResponse, QueryJobResult},
};
use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use uuid::Uuid;

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

    let req_data = body.to_owned();
    println!("{}", req_data.query.as_str());

    let ctx = data.ballista_context.lock().unwrap();
    let df = match ctx.sql(req_data.query.as_str()).await {
        Ok(result) => result,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(ErrorResponse::InternalServerError(e.to_string()));
        }
    };

    let result = match df.collect().await {
        Ok(result) => result,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .json(ErrorResponse::InternalServerError(e.to_string()));
        }
    };

    if result.len() == 0 {
        return HttpResponse::Ok().json(QueryJobResponse {
            message: "success".to_string(),
            job_type: "job".to_string(),
            result: QueryJobResult {
                total_rows: 0,
                columns: Vec::new(),
            },
        });
    }

    let record_batch = result[0].clone();
    let query_result: QueryJobResult = QueryJobResult {
        total_rows: record_batch.num_rows() as u32,
        columns: record_batch_to_vec(record_batch),
    };

    let json_response = QueryJobResponse {
        message: "success".to_string(),
        job_type: "table".to_string(),
        result: query_result,
    };

    HttpResponse::Ok().json(json_response)
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api").service(create_query_job_handler);

    conf.service(health_checker_handler).service(scope);
}
