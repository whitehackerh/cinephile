use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::usecases::security::token::TokenManager;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

pub struct JwtTokenManager {
    secret: String,
}

impl JwtTokenManager {
    pub fn new(secret: String) -> Self {
        Self { secret }
    }
}

impl TokenManager for JwtTokenManager {
    fn generate(&self, user_id: Uuid) -> Result<String, String> {
        let now = Utc::now();
        let expiration = now
            .checked_add_signed(Duration::days(7))
            .ok_or("Invalid expiration time")?
            .timestamp();

        let claims = Claims {
            sub: user_id.to_string(),
            iat: now.timestamp() as usize,
            exp: expiration as usize,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .map_err(|e| format!("Token generation failed: {}", e))
    }
}