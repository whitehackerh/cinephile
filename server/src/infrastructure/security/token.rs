use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::usecases::security::token::TokenManager;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Claims {
    pub sub: String,
    pub iat: u64,
    pub exp: u64,
}

pub(crate) struct JwtTokenManager {
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
        let iat = u64::try_from(now.timestamp())
            .map_err(|_| "Current time is before Unix epoch".to_string())?;
        let exp = u64::try_from(
            now.checked_add_signed(Duration::days(7))
                .ok_or_else(|| "Invalid expiration time".to_string())?
                .timestamp(),
        )
        .map_err(|_| "Expiration time is before Unix epoch".to_string())?;

        let claims = Claims {
            sub: user_id.to_string(),
            iat,
            exp,
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_ref()),
        )
        .map_err(|e| format!("Token generation failed: {}", e))
    }
}