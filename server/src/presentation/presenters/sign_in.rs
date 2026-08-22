use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct SignInRequest {
    pub email: String,
    pub password: String,
}
