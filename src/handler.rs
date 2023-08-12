use crate::{
    model::{AppState, QueryJob},
    response::{GenericResponse, QueryJobData, QueryJobResponse},
};
use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use uuid::Uuid;

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
    let mut vec = data.jobs_db.lock().unwrap();
    let uuid_id = Uuid::new_v4();
    let datetime = Utc::now();

    body.id = Some(uuid_id.to_string());
    body.completed = Some(false);
    body.createdAt = Some(datetime);

    let job = body.to_owned();

    println!("{:?}", body);

    vec.push(body.into_inner());

    let json_response = QueryJobResponse {
        status: "success".to_string(),
        result: QueryJobData { job },
    };

    HttpResponse::Ok().json(json_response)
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(create_query_job_handler);

    conf.service(scope);
}
