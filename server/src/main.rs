use axum::{Router};
use std::env;
use serde_json::json;
use std::net::SocketAddr;
use sqlx::postgres::PgPoolOptions;
use tower_http::services::ServeDir;
use utoipa_swagger_ui::{SwaggerUi, Config};
use server::{AppRegistry, AppState};
use server::infrastructure::ui::router::create_router;

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

    let listener = tokio::net::TcpListener::bind(addr).await?;

    let app = create_router(shared_state)
        .merge(SwaggerUi::new("/swagger-ui")
        .config(Config::from("/doc/openapi.yaml")))
        .nest_service("/doc", ServeDir::new("../doc"));

    println!("🚀 Server started at http://{}", addr);
    println!("📖 Swagger UI available at http://{}/swagger-ui", addr);

    axum::serve(listener, app).await?;

    Ok(())
}
