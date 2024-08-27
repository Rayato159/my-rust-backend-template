use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub trait CustomError {
    fn error(&self) -> ErrorResponse;
}
