use crate::{
    entities::user::User,
    models::{error::CustomError, user::Error},
};
use axum::async_trait;
use mockall::*;
use sqlx::PgPool;
use std::sync::Arc;
use tracing::error;

pub type SharedUsersRepository = Arc<dyn UsersRepository + Send + Sync>;

#[async_trait]
#[automock]
pub trait UsersRepository {
    async fn inserting(&self, user: User) -> Result<i32, Box<dyn CustomError>>;
    async fn selecting_by_id(&self, id: i32) -> Result<User, Box<dyn CustomError>>;
    async fn selecting_by_username(&self, username: &str) -> Result<User, sqlx::Error>;
}

pub struct UsersRepositoryImpl {
    db_pool: PgPool,
}

impl UsersRepositoryImpl {
    pub fn creation(db_pool: PgPool) -> SharedUsersRepository {
        Arc::new(Self { db_pool })
    }
}

#[async_trait]
impl UsersRepository for UsersRepositoryImpl {
    async fn inserting(&self, user: User) -> Result<i32, Box<dyn CustomError>> {
        let user = match sqlx::query_as::<_, User>(
            "INSERT INTO users (username, password, created_at, updated_at) VALUES ($1, $2, $3, $4) RETURNING *;",
        )
        .bind(user.username)
        .bind(user.password)
        .bind(user.created_at)
        .bind(user.updated_at)
        .fetch_one(&self.db_pool)
        .await
        {
            Ok(user) => user,
            Err(e) => {
                error!("{:?}", e);
                return Err(Box::new(Error::Inserting));
            }
        };

        Ok(match user.id {
            Some(id) => id,
            None => {
                error!("Failed to get id");
                return Err(Box::new(Error::Inserting));
            }
        })
    }

    async fn selecting_by_id(&self, id: i32) -> Result<User, Box<dyn CustomError>> {
        let user = match sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1;")
            .bind(id)
            .fetch_one(&self.db_pool)
            .await
        {
            Ok(user) => user,
            Err(e) => {
                error!("{:?}", e);
                return Err(Box::new(Error::NotFound(id)));
            }
        };

        Ok(user)
    }

    async fn selecting_by_username(&self, username: &str) -> Result<User, sqlx::Error> {
        let user = match sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = $1;")
            .bind(username)
            .fetch_one(&self.db_pool)
            .await
        {
            Ok(user) => user,
            Err(e) => {
                return Err(e);
            }
        };

        Ok(user)
    }
}
