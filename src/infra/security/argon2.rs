use argon2::{
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
    password_hash::{SaltString, rand_core::OsRng},
};

pub trait PasswordHasherTrait: Send + Sync {
    fn hash_password(&self, password: &str) -> Result<String, String>;
    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, String>;
}

#[derive(Default)]
pub struct Argon2PasswordHasher {
    hasher: Argon2<'static>,
}

impl PasswordHasherTrait for Argon2PasswordHasher {
    fn hash_password(&self, password: &str) -> Result<String, String> {
        let salt = SaltString::generate(&mut OsRng);

        self.hasher
            .hash_password(password.as_bytes(), &salt)
            .map(|hash| hash.to_string())
            .map_err(|e| e.to_string())
    }

    fn verify_password(&self, password: &str, hash: &str) -> Result<bool, String> {
        let parsed_hash = PasswordHash::new(hash).map_err(|e| e.to_string())?;

        self.hasher
            .verify_password(password.as_bytes(), &parsed_hash)
            .map(|_| true)
            .map_err(|e| e.to_string())
    }
}
