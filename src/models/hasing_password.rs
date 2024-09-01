use super::error::{CustomError, ErrorResponse};
use axum::http::StatusCode;

pub enum Error {
    PasswordHashing,
}

impl CustomError for Error {
    fn error(&self) -> ErrorResponse {
        match self {
            Error::PasswordHashing => ErrorResponse {
                error: "Password hashing failed".to_string(),
                status_code: StatusCode::INTERNAL_SERVER_ERROR,
            },
        }
    }
}
