use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
pub struct PasswordManager;

impl PasswordManager {
    pub fn hash(password: &str) -> Result<String, String> {
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        
        argon2
            .hash_password(password.as_bytes(), &salt)
            .map(|h| h.to_string())
            .map_err(|_| "Internal security error".to_string())
    }

    pub fn verify(password: &str, hash: &str) -> bool {
        use argon2::PasswordHash;
        let parsed_hash = PasswordHash::new(hash).unwrap();
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed_hash)
            .is_ok()
    }
}
