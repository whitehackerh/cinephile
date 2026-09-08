use axum::http::HeaderValue;
use std::env;
use std::net::SocketAddr;
use sqlx::postgres::PgPoolOptions;
use tower_http::services::ServeDir;
use tower_http::cors::CorsLayer;
use utoipa_swagger_ui::{SwaggerUi, Config};
use server::{AppRegistry, infrastructure::ui::router::create_router};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;
    let shared_state = AppRegistry::build(pool).await;

    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:3000".parse::<HeaderValue>().unwrap(),
            "http://127.0.0.1:8080".parse::<HeaderValue>().unwrap(),
            "http://localhost:8080".parse::<HeaderValue>().unwrap(),
        ])
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST, axum::http::Method::PATCH, axum::http::Method::PUT, axum::http::Method::DELETE, axum::http::Method::OPTIONS])
        .allow_headers([axum::http::header::CONTENT_TYPE, axum::http::header::AUTHORIZATION])
        .expose_headers([axum::http::header::AUTHORIZATION]);

    let listener = tokio::net::TcpListener::bind(addr).await?;

    let app = create_router(shared_state)
        .merge(SwaggerUi::new("/swagger-ui")
        .config(Config::from("/doc/openapi.yaml")))
        .nest_service("/doc", ServeDir::new(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../doc")))
        .layer(cors);

    println!("🚀 Server started at http://{}", addr);
    println!("📖 Swagger UI available at http://{}/swagger-ui", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
