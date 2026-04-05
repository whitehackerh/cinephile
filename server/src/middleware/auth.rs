use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use axum_extra::extract::TypedHeader;
use axum_extra::headers::{authorization::Bearer, Authorization};
use std::sync::Arc;
use crate::infrastructure::security::token::JwtTokenManager;
use crate::domain::entities::auth_user::AuthUser;

pub(crate) struct AuthMiddleware;

impl AuthMiddleware {
    pub async fn auth_middleware(
        State(token_manager): State<Arc<JwtTokenManager>>, 
        auth_header: Option<TypedHeader<Authorization<Bearer>>>,
        mut request: Request,
        next: Next,
    ) -> Result<Response, StatusCode> {
        let auth_header = auth_header.ok_or(StatusCode::UNAUTHORIZED)?;
        let token = auth_header.token();

        let user_id = token_manager
            .verify_and_extract(token)
            .map_err(|_| StatusCode::UNAUTHORIZED)?;

        request.extensions_mut().insert(AuthUser::new(user_id, token.to_string()));

        Ok(next.run(request).await)
    }
}
