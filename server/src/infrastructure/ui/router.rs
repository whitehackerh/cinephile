use axum::{routing::{get, post}, Router, middleware::from_fn_with_state};
use std::sync::Arc;
use crate::AppRegistry;
use crate::AppState;
use crate::handlers::sign_up::signup_handler;
use crate::handlers::sign_in::signin_handler;
// use crate::infrastructure::middleware::auth_guard;

pub fn create_router(registry: Arc<AppRegistry>) -> Router {
    let public_routes = Router::<AppState>::new()
            .route("/signup", post(signup_handler))
            .route("/signin", post(signin_handler));

    // let protected_routes = Router::new()
    //     .route("/me", get(crate::handlers::user::me_handler))
    //     .layer(from_fn_with_state(shared_state.clone(), auth_guard));

    Router::new()
        .nest("/api", public_routes)
        // .nest("/api/auth", protected_routes)
        .with_state(AppState(registry))
}
