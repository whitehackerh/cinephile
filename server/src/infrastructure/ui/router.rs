use axum::{routing::{get, post}, Router, middleware::from_fn_with_state};
use std::sync::Arc;

use crate::{
    AppRegistry,
    AppState,
    presentation::handlers::{
        sign_up::signup_handler,
        sign_in::signin_handler,
        search::search_handler,
        movie::movie_handler,
        tv_episode::tv_episode_handler,
        tv_season::tv_season_handler,
        tv_series::tv_series_handler,
        post_reviews::post_reviews_handler,
        get_review::get_review_handler,
    },
    middleware::auth::AuthMiddleware
};

pub fn create_router(registry: Arc<AppRegistry>) -> Router {
    let protected_routes = Router::new()
        .route("/search", get(search_handler))
        .route("/movie/{id}", get(movie_handler))
        .route("/tv/{series_id}/season/{season_number}/episode/{episode_number}", get(tv_episode_handler))
        .route("/tv/{series_id}/season/{season_number}", get(tv_season_handler))
        .route("/tv/{series_id}", get(tv_series_handler))
        .route("/reviews", post(post_reviews_handler))
        .route("/reviews/find", get(get_review_handler))
        .layer(from_fn_with_state(registry.token_manager.clone(), AuthMiddleware::auth_middleware));
    
    let public_routes = Router::<AppState>::new()
            .route("/signup", post(signup_handler))
            .route("/signin", post(signin_handler));

    Router::new()
        .nest("/api", public_routes.merge(protected_routes))
        .with_state(AppState(registry))
}
