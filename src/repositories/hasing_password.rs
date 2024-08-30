use crate::models::{error::CustomError, hasing_password::Error};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use axum::async_trait;
use mockall::*;
use std::sync::Arc;
use tracing::error;

pub type SharedHashingPassword = Arc<dyn HashingPassword + Send + Sync>;

#[async_trait]
#[automock]
pub trait HashingPassword {
    async fn hasing(&self, password: String) -> Result<String, Box<dyn CustomError>>;
}

pub struct HashingPasswordImpl;

impl HashingPasswordImpl {
    pub fn creation() -> SharedHashingPassword {
        Arc::new(Self)
    }
}

#[async_trait]
impl HashingPassword for HashingPasswordImpl {
    async fn hasing(&self, password: String) -> Result<String, Box<dyn CustomError>> {
        let salt = SaltString::generate(&mut OsRng);

        let argon2 = Argon2::default();

        let password_hash = match argon2.hash_password(password.as_bytes(), &salt) {
            Ok(hash) => hash,
            Err(e) => {
                error!("{:?}", e);
                return Err(Box::new(Error::PasswordHashing));
            }
        };

        Ok(password_hash.to_string())
    }
}
