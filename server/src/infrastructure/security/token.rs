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

#[cfg(test)]
mod tests {
    use super::*;
    use jsonwebtoken::{decode, DecodingKey, Validation};
    use crate::usecases::security::token::TokenManager;

    fn make_manager() -> JwtTokenManager {
        JwtTokenManager::new("test_secret_key_for_unit_tests".to_string())
    }

    #[test]
    fn generate_returns_non_empty_token() {
        let manager = make_manager();
        let user_id = Uuid::new_v4();
        let result = manager.generate(user_id);
        assert!(result.is_ok());
        assert!(!result.unwrap().is_empty());
    }

    #[test]
    fn generated_token_has_three_jwt_parts() {
        let manager = make_manager();
        let user_id = Uuid::new_v4();
        let token = manager.generate(user_id).unwrap();
        let parts: Vec<&str> = token.split('.').collect();
        assert_eq!(parts.len(), 3, "JWT should have header.payload.signature parts");
    }

    #[test]
    fn generated_token_contains_correct_subject() {
        let manager = make_manager();
        let user_id = Uuid::new_v4();
        let token = manager.generate(user_id).unwrap();

        let mut validation = Validation::default();
        validation.validate_exp = false;
        let data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret("test_secret_key_for_unit_tests".as_ref()),
            &validation,
        )
        .expect("Token should be decodable");

        assert_eq!(data.claims.sub, user_id.to_string());
    }

    #[test]
    fn generated_token_expiration_is_7_days_from_now() {
        let manager = make_manager();
        let user_id = Uuid::new_v4();
        let before = Utc::now().timestamp() as usize;
        let token = manager.generate(user_id).unwrap();
        let after = Utc::now().timestamp() as usize;

        let mut validation = Validation::default();
        validation.validate_exp = false;
        let data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret("test_secret_key_for_unit_tests".as_ref()),
            &validation,
        )
        .unwrap();

        let seven_days_secs = 7 * 24 * 60 * 60;
        let min_exp = before + seven_days_secs;
        let max_exp = after + seven_days_secs;
        assert!(data.claims.exp >= min_exp);
        assert!(data.claims.exp <= max_exp);
    }

    #[test]
    fn token_fails_to_decode_with_wrong_secret() {
        let manager = make_manager();
        let user_id = Uuid::new_v4();
        let token = manager.generate(user_id).unwrap();

        let mut validation = Validation::default();
        validation.validate_exp = false;
        let result = decode::<Claims>(
            &token,
            &DecodingKey::from_secret("wrong_secret".as_ref()),
            &validation,
        );
        assert!(result.is_err(), "Token with wrong secret should fail validation");
    }

    #[test]
    fn different_user_ids_produce_different_tokens() {
        let manager = make_manager();
        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();
        let token1 = manager.generate(id1).unwrap();
        let token2 = manager.generate(id2).unwrap();
        assert_ne!(token1, token2);
    }

    #[test]
    fn generate_with_empty_secret_still_produces_token() {
        let manager = JwtTokenManager::new(String::new());
        let user_id = Uuid::new_v4();
        let result = manager.generate(user_id);
        assert!(result.is_ok(), "Should produce token even with empty secret");
    }

    #[test]
    fn claims_iat_is_approximately_now() {
        let manager = make_manager();
        let user_id = Uuid::new_v4();
        let before = Utc::now().timestamp() as usize;
        let token = manager.generate(user_id).unwrap();
        let after = Utc::now().timestamp() as usize;

        let mut validation = Validation::default();
        validation.validate_exp = false;
        let data = decode::<Claims>(
            &token,
            &DecodingKey::from_secret("test_secret_key_for_unit_tests".as_ref()),
            &validation,
        )
        .unwrap();

        assert!(data.claims.iat >= before);
        assert!(data.claims.iat <= after);
    }
}