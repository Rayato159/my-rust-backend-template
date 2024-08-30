use crate::{
    entities::user::User as UserEntity,
    models::{
        error::CustomError,
        user::{Error, User, UserRegistration},
    },
    repositories::{hasing_password::SharedHashingPassword, users::SharedUsersRepository},
};
use axum::async_trait;
use std::sync::Arc;

#[async_trait]
pub trait UsersService {
    async fn registration(
        &self,
        user: &UserRegistration,
        is_auto_time_needed: bool,
    ) -> Result<User, Box<dyn CustomError>>;
}

pub type SharedUsersService = Arc<dyn UsersService + Send + Sync>;

pub struct UsersServiceImpl {
    users_repository: SharedUsersRepository,
    hashing_password: SharedHashingPassword,
}

impl UsersServiceImpl {
    pub fn creation(
        users_repository: SharedUsersRepository,
        hashing_password: SharedHashingPassword,
    ) -> SharedUsersService {
        Arc::new(Self {
            users_repository,
            hashing_password,
        })
    }
}

#[async_trait]
impl UsersService for UsersServiceImpl {
    async fn registration(
        &self,
        user: &UserRegistration,
        is_auto_time_needed: bool,
    ) -> Result<User, Box<dyn CustomError>> {
        if let Ok(r) = self
            .users_repository
            .selecting_by_username(&user.username)
            .await
        {
            return Err(Box::new(Error::AlreadyExists(r.username)));
        }

        let hashed_password = match self.hashing_password.hasing(user.password.clone()).await {
            Ok(hashed) => hashed,
            Err(e) => return Err(e),
        };

        let user_id = match self
            .users_repository
            .inserting(UserEntity::new(
                user.username.clone(),
                hashed_password.clone(),
                is_auto_time_needed,
            ))
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

#[cfg(test)]
mod tests {
    use core::panic;
    use std::sync::Arc;

    use chrono::{TimeZone, Utc};
    use mockall::predicate::eq;

    use crate::entities::user::User as UserEntity;
    use crate::models::user::{User, UserRegistration};
    use crate::repositories::hasing_password::MockHashingPassword;
    use crate::repositories::users::MockUsersRepository;

    use super::UsersServiceImpl;

    #[tokio::test]
    async fn registration_test() {
        let mut users_repository_mock = MockUsersRepository::new();
        let mut hashing_password_mock = MockHashingPassword::new();

        let req = UserRegistration {
            username: "test".to_string(),
            password: "123456".to_string(),
        };

        let expected_user = User {
            id: 1,
            username: "test".to_string(),
        };

        users_repository_mock
            .expect_selecting_by_username()
            .with(eq("test"))
            .times(1)
            .returning(|_| Box::pin(async { Err(sqlx::Error::RowNotFound) }));

        hashing_password_mock
            .expect_hasing()
            .with(eq("123456".to_string()))
            .returning(|_| Box::pin(async { Ok("xxxxx".to_string()) }));

        users_repository_mock
            .expect_inserting()
            .with(eq(UserEntity {
                id: None,
                username: "test".to_string(),
                password: "xxxxx".to_string(),
                created_at: Utc
                    .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
                    .unwrap()
                    .naive_utc(),
                updated_at: Utc
                    .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
                    .unwrap()
                    .naive_utc(),
            }))
            .returning(|_| Box::pin(async { Ok(1) }));

        users_repository_mock
            .expect_selecting_by_id()
            .with(eq(1))
            .returning(|_| {
                Box::pin(async {
                    Ok(UserEntity {
                        id: Some(1),
                        username: "test".to_string(),
                        password: "xxxxx".to_string(),
                        created_at: Utc
                            .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
                            .unwrap()
                            .naive_utc(),
                        updated_at: Utc
                            .with_ymd_and_hms(1970, 1, 1, 0, 0, 0)
                            .unwrap()
                            .naive_utc(),
                    })
                })
            });

        let users_service = UsersServiceImpl::creation(
            Arc::new(users_repository_mock),
            Arc::new(hashing_password_mock),
        );

        let user_result = match users_service.registration(&req, false).await {
            Ok(user) => user,
            Err(e) => panic!("{:?}", e.error()),
        };

        // Use assert_eq! to compare the expected and actual users
        assert_eq!(user_result.id, expected_user.id);
        assert_eq!(user_result.username, expected_user.username);
    }
}
