use serde::{Deserialize, Serialize};

use super::error::{CustomError, ErrorResponse};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub password: String,
}

#[derive(Debug)]
pub struct UserNotFoundError {
    pub id: i64,
}

impl UserNotFoundError {
    pub fn new(id: i64) -> Self {
        Self { id }
    }
}

impl CustomError for UserNotFoundError {
    fn error(&self) -> ErrorResponse {
        ErrorResponse {
            error: format!("User with id {} not found", self.id),
        }
    }
}
