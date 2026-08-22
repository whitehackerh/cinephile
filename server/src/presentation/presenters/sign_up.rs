use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct SignUpRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}