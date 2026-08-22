use axum::{
    extract::{State, Query},
    http::{StatusCode, Uri},
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use crate::{
    generated::api_schema::SearchQueryParams,
    presentation::presenters::{
        base_response::ApiResponse,
        search::SearchPresenter
    },
    usecases::{
        port::search::SearchUseCase,
        dto::search::SearchInput
    }
};

pub async fn search_handler(
    uri: Uri,
    State(usecase): State<Arc<dyn SearchUseCase + Send + Sync>>,
    Query(params): Query<SearchQueryParams>,
) -> impl IntoResponse {
    let input = SearchInput {
        query: params.q.into(),
        page: params.page.get() as u32,
    };

    match usecase.execute(input).await {
        Ok(output) => {
            (
                StatusCode::OK,
                Json(ApiResponse::success(uri.to_string(), SearchPresenter::to_response(output)))
            ).into_response()
        },
        Err(e) => ApiResponse::from_error(&uri, e).into_response(),
    }
}
