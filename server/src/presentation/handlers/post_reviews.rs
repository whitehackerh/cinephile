// use axum::{
//     extract::State,
//     http::{StatusCode, Uri},
//     response::IntoResponse,
//     Json,
// };
// use std::sync::Arc;
// use serde_json::{Value};

// use crate::{
//     presentation::presenters::{
//         base_response::ApiResponse,
//         post_reviews::{
//             PostReviewsRequest,
//             PostReviewsResponse
//         }
//     },
//     usecases::{
//         dto::post_reviews::PostReviewsInput,
//         port::post_reviews::PostReviewsUseCase
//     }
// };

// pub async fn post_reviews_handler(
//     uri: Uri,
//     State(usecase): State<Arc<dyn PostReviewsUseCase + Send + Sync>>,
//     Json(payload): Json<PostReviewsRequest>,
// ) -> impl IntoResponse {
//     match usecase.execute(PostReviewsInput {
//         rating: payload.rating,
//         content: payload.content,
//         work_type: payload.work_type,
//         target_path: payload.target_path
//     }).await {
//         Ok(output) => {
//             (
//                 StatusCode::OK,
//                 Json(ApiResponse::success(uri.to_string(), PostReviewsResponse::from(output)))
//             ).into_response()
//         },
//         Err(e) => ApiResponse::<Value>::from_error(&uri, e).into_response(),
//     }
// }