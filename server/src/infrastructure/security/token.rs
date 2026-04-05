use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
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
    decoding_key: DecodingKey,
    encoding_key: EncodingKey,
}

impl JwtTokenManager {
    pub fn new(secret: String) -> Self {
        let bytes = secret.as_bytes();
        Self {
            decoding_key: DecodingKey::from_secret(bytes),
            encoding_key: EncodingKey::from_secret(bytes),
        }
    }

    pub fn verify_and_extract(&self, token: &str) -> Result<Uuid, jsonwebtoken::errors::Error> {
        let validation = Validation::default();
        let token_data = decode::<Claims>(
            token,
            &self.decoding_key,
            &validation,
        )?;

        Uuid::parse_str(&token_data.claims.sub)
            .map_err(|_| jsonwebtoken::errors::ErrorKind::InvalidToken.into())
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
            &self.encoding_key,
        )
        .map_err(|e| format!("Token generation failed: {}", e))
    }
}
