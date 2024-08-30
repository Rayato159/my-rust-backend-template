use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Json};
use sqlx::PgPool;

use crate::{
    models::user::UserRegistration,
    repositories::{hasing_password::HashingPasswordImpl, users::UsersRepositoryImpl},
    services::users::{SharedUsersService, UsersServiceImpl},
};

pub struct UsersHandler {
    pub users_service: SharedUsersService,
}

impl UsersHandler {
    pub fn new(db_pool: PgPool) -> Self {
        let users_repository = UsersRepositoryImpl::creation(db_pool.clone());
        let hasing_password = HashingPasswordImpl::creation();

        Self {
            users_service: UsersServiceImpl::creation(
                Arc::clone(&users_repository),
                Arc::clone(&hasing_password),
            ),
        }
    }
}

pub async fn registration(
    Json(req): Json<UserRegistration>,
    users_service: SharedUsersService,
) -> impl IntoResponse {
    let user = match users_service.registration(&req, true).await {
        Ok(user) => user,
        Err(e) => return e.error().into_response(),
    };

    (StatusCode::CREATED, Json(user)).into_response()
}
