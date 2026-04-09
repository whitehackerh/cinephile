use axum::{routing::{get, post}, Router, middleware::from_fn_with_state};
use std::sync::Arc;

use crate::{
    AppRegistry,
    AppState,
    handlers::{
        sign_up::signup_handler,
        sign_in::signin_handler,
        search::search_handler,
        movie::movie_handler,
    },
    middleware::auth::AuthMiddleware
};

pub fn create_router(registry: Arc<AppRegistry>) -> Router {
    let protected_routes = Router::new()
        .route("/search", get(search_handler))
        .route("/movie/{id}", get(movie_handler))
        .layer(from_fn_with_state(AppState(registry.clone()), AuthMiddleware::auth_middleware));
    
    let public_routes = Router::<AppState>::new()
            .route("/signup", post(signup_handler))
            .route("/signin", post(signin_handler));

    Router::new()
        .nest("/api", public_routes.merge(protected_routes))
        .with_state(AppState(registry))
}
