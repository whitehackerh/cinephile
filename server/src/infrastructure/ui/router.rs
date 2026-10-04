use axum::{routing::{get, patch, post }, Router, middleware::from_fn_with_state};
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
        create_review::create_review_handler,
        update_review::update_review_handler,
        delete_reviews::delete_reviews_handler,
        get_reviews::get_reviews_handler,
        get_review::get_review_handler,
        create_collection::create_collection_handler,
        add_collection_work::add_collection_work_handler,
        get_collections::get_collections_handler
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
        .route("/reviews", post(create_review_handler).get(get_reviews_handler))
        .route("/reviews/{id}", patch(update_review_handler).delete(delete_reviews_handler))
        .route("/reviews/find", get(get_review_handler))
        .route("/collections", post(create_collection_handler).get(get_collections_handler))
        .route("/collections/{id}/works", post(add_collection_work_handler))
        .layer(from_fn_with_state(registry.token_manager.clone(), AuthMiddleware::auth_middleware));
    
    let public_routes = Router::<AppState>::new()
            .route("/signup", post(signup_handler))
            .route("/signin", post(signin_handler));

    Router::new()
        .nest("/api", public_routes.merge(protected_routes))
        .with_state(AppState(registry))
}
