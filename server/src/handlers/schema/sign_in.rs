use serde::Deserialize;

#[derive(Deserialize)]
pub struct SignInRequest {
    pub email: String,
    pub password: String,
}
