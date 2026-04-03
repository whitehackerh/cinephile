pub trait PasswordManager: Send + Sync {
    fn hash(&self, password: &str) -> Result<String, String>;
    fn verify(&self, password: &str, hash: &str) -> bool;
}