use axum::{routing::{get, post}, Router, middleware::from_fn_with_state};
use std::sync::Arc;
use crate::AppRegistry;
use crate::AppState;
use crate::handlers::sign_up::signup_handler;
use crate::handlers::sign_in::signin_handler;
use crate::middleware::auth::AuthMiddleware;

pub fn create_router(registry: Arc<AppRegistry>) -> Router {
    let protected_routes = Router::new()
        // .route("/xx", get(xxxx))
        .layer(from_fn_with_state(AppState(registry.clone()), AuthMiddleware::auth_middleware));
    
    let public_routes = Router::<AppState>::new()
            .route("/signup", post(signup_handler))
            .route("/signin", post(signin_handler));

    Router::new()
        .nest("/api", public_routes.merge(protected_routes))
        .with_state(AppState(registry))
}
