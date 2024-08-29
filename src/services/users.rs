use std::sync::Arc;

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};

use axum::async_trait;
use tracing::error;

use crate::{
    entities::user::User as UserEntity,
    models::{
        error::CustomError,
        user::{Error, User, UserRegistration},
    },
    repositories::users::SharedUsersRepository,
};

#[async_trait]
pub trait UsersService {
    async fn registration(&self, user: &UserRegistration) -> Result<User, Box<dyn CustomError>>;
}

pub type SharedUsersService = Arc<Box<dyn UsersService + Send + Sync>>;

pub struct UsersServiceImpl {
    users_repository: SharedUsersRepository,
}

impl UsersServiceImpl {
    pub fn creation(users_repository: SharedUsersRepository) -> SharedUsersService {
        Arc::new(Box::new(Self { users_repository }))
    }

    fn password_hasing(password: String) -> Result<String, Box<dyn CustomError>> {
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

#[async_trait]
impl UsersService for UsersServiceImpl {
    async fn registration(&self, user: &UserRegistration) -> Result<User, Box<dyn CustomError>> {
        match self
            .users_repository
            .selecting_by_username(&user.username)
            .await
        {
            Ok(r) => return Err(Box::new(Error::AlreadyExists(r.username))),
            Err(_) => (),
        }

        let password_hash = match Self::password_hasing(user.password.clone()) {
            Ok(password) => password,
            Err(e) => return Err(e),
        };

        let user_id = match self
            .users_repository
            .inserting(&UserEntity::new(user.username.clone(), password_hash))
            .await
        {
            Ok(id) => id,
            Err(e) => return Err(e),
        };

        let user = match self.users_repository.selecting_by_id(user_id).await {
            Ok(user) => user,
            Err(e) => return Err(e),
        };

        Ok(user.to_model())
    }
}
