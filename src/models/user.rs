use axum::{http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};

use super::error::{CustomError, ErrorResponse};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserRegistration {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i32,
    pub username: String,
}

impl IntoResponse for User {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        let body = axum::Json(&self);
        (StatusCode::CREATED, body).into_response()
    }
}

pub enum Error {
    NotFound(i32),
    Inserting,
    PasswordHashing,
    AlreadyExists(String),
}

impl CustomError for Error {
    fn error(&self) -> ErrorResponse {
        match self {
            Error::NotFound(id) => ErrorResponse {
                error: format!("User not found: {}", id),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            Error::Inserting => ErrorResponse {
                error: "User creation failed".to_string(),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            Error::PasswordHashing => ErrorResponse {
                error: "Password hashing failed".to_string(),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
            Error::AlreadyExists(username) => ErrorResponse {
                error: format!("Username already exists: {}", username),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
        }
    }
}
