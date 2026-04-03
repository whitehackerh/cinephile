use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use crate::usecases::security::password::PasswordManager as PasswordManagerTrait;
pub struct PasswordManager;

impl PasswordManager {
    pub fn new() -> Self {
        Self
    }
}

impl PasswordManagerTrait for PasswordManager {
    fn hash(&self, password: &str) -> Result<String, String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|h| h.to_string())
            .map_err(|_| "Internal security error".to_string())
    }

    fn verify(&self, password: &str, hash: &str) -> bool {
        use argon2::PasswordHash;
        let parsed_hash = PasswordHash::new(hash).unwrap();
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::usecases::security::password::PasswordManager as PasswordManagerTrait;

    #[test]
    fn hash_returns_non_empty_string() {
        let manager = PasswordManager::new();
        let result = manager.hash("my_password");
        assert!(result.is_ok());
        let hash = result.unwrap();
        assert!(!hash.is_empty());
    }

    #[test]
    fn hash_produces_argon2_formatted_output() {
        let manager = PasswordManager::new();
        let hash = manager.hash("password123").unwrap();
        assert!(hash.starts_with("$argon2id$"));
    }

    #[test]
    fn hash_produces_different_values_for_same_password() {
        let manager = PasswordManager::new();
        let hash1 = manager.hash("same_password").unwrap();
        let hash2 = manager.hash("same_password").unwrap();
        assert_ne!(hash1, hash2, "Each hash should have a unique salt");
    }

    #[test]
    fn verify_returns_true_for_correct_password() {
        let manager = PasswordManager::new();
        let hash = manager.hash("correct_password").unwrap();
        assert!(manager.verify("correct_password", &hash));
    }

    #[test]
    fn verify_returns_false_for_wrong_password() {
        let manager = PasswordManager::new();
        let hash = manager.hash("correct_password").unwrap();
        assert!(!manager.verify("wrong_password", &hash));
    }

    #[test]
    fn verify_returns_false_for_empty_password_against_non_empty_hash() {
        let manager = PasswordManager::new();
        let hash = manager.hash("nonempty").unwrap();
        assert!(!manager.verify("", &hash));
    }

    #[test]
    fn hash_and_verify_work_for_empty_password() {
        let manager = PasswordManager::new();
        let hash = manager.hash("").unwrap();
        assert!(manager.verify("", &hash));
        assert!(!manager.verify("not_empty", &hash));
    }

    #[test]
    fn verify_returns_false_for_similar_but_different_password() {
        let manager = PasswordManager::new();
        let hash = manager.hash("Password1").unwrap();
        assert!(!manager.verify("password1", &hash));
        assert!(!manager.verify("Password1 ", &hash));
    }
}