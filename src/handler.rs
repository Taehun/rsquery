use crate::{
    model::{AppState, QueryJob},
    response::{
        ErrorResponse, GenericResponse, QueryJobResponse, QueryJobResponseString, QueryJobResult,
    },
};
use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use datafusion::arrow::{
    array::{
        Float32Array, Float64Array, Int16Array, Int32Array, Int64Array, Int8Array, StringArray,
        UInt64Array, UInt8Array,
    },
    datatypes::*,
    record_batch::RecordBatch,
};
use uuid::Uuid;

fn record_batch_to_vec(batch: RecordBatch) -> Vec<Vec<String>> {
    let mut result = Vec::new();

    for i in 0..batch.num_columns() {
        let column = batch.column(i);
        let mut values = Vec::new();

        for j in 0..column.len() {
            if column.is_null(j) {
                values.push("".to_string());
            } else {
                match column.data_type() {
                    DataType::Utf8 => {
                        let array = column.as_any().downcast_ref::<StringArray>().unwrap();
                        values.push(array.value(j).to_string());
                    }
                    DataType::Int32 => {
                        let array = column.as_any().downcast_ref::<Int32Array>().unwrap();
                        values.push(array.value(j).to_string());
                    }
                    DataType::Int16 => {
                        let array = column.as_any().downcast_ref::<Int16Array>().unwrap();
                        values.push(array.value(j).to_string());
                    }
                    DataType::Int8 => {
                        let array = column.as_any().downcast_ref::<Int8Array>().unwrap();
                        values.push(array.value(j).to_string());
                    }
                    DataType::Int64 => {
                        let array = column.as_any().downcast_ref::<Int64Array>().unwrap();
                        values.push(array.value(j).to_string());
                    }
                    DataType::Float32 => {
                        let array = column.as_any().downcast_ref::<Float32Array>().unwrap();
                        values.push(array.value(j).to_string());
                    }
                    DataType::Float64 => {
                        let array = column.as_any().downcast_ref::<Float64Array>().unwrap();
                        values.push(array.value(j).to_string());
                    }
                    DataType::UInt8 => {
                        let array: &datafusion::arrow::array::PrimitiveArray<UInt8Type> =
                            column.as_any().downcast_ref::<UInt8Array>().unwrap();
                        values.push(array.value(j).to_string());
                    }
                    DataType::UInt64 => {
                        let array = column.as_any().downcast_ref::<UInt64Array>().unwrap();
                        values.push(array.value(j).to_string());
                    }
                    _ => {
                        values.push("ERROR".to_string()); // Handle unknown data types
                    }
                }
            }
        }

        result.push(values);
    }

    result
}

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
        return HttpResponse::Ok().json(QueryJobResponseString {
            status: "success".to_string(),
            result: "Query job is success".to_string(),
        });
    }

    let record_batch = result[0].clone();
    let query_result: QueryJobResult = QueryJobResult {
        total_rows: record_batch.num_rows() as u32,
        columns: record_batch_to_vec(record_batch),
    };

    let json_response = QueryJobResponse {
        status: "success".to_string(),
        result: query_result,
    };

    HttpResponse::Ok().json(json_response)
}

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api").service(create_query_job_handler);

    conf.service(health_checker_handler).service(scope);
}
