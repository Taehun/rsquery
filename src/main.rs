mod handle_result;
mod handler;
mod model;
mod response;

use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{http::header, web, App, HttpServer};
use model::AppState;
use response::*;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    #[derive(OpenApi)]
    #[openapi(
        paths(
            handler::health_checker_handler,
            handler::create_query_job_handler,
        ),
        components(
            schemas(
                model::QueryJobRequest, model::QueryJob,
                QueryJobData, QueryJobResponse, QueryJobResult, GenericResponse, ErrorResponse, Schema
            )
        ),
        tags(
            (name = "RSQuery", description = "RSQuery REST API endpoints.")
        ),
    )]
    struct ApiDoc;

    let ballista_ctx = AppState::init().await;
    let app_data: web::Data<AppState> = web::Data::new(ballista_ctx);
    let openapi: utoipa::openapi::OpenApi = ApiDoc::openapi();

    println!("🚀 Server started successfully");

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_origin("http://localhost:8080")
            .allowed_origin("https://rsquery.taehun.dev")
            .allowed_origin("https://rsquery-api.taehun.dev")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .app_data(app_data.clone())
            .configure(handler::config)
            .wrap(cors)
            .wrap(Logger::default())
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
