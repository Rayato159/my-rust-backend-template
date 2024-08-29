use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug)]
pub struct ErrorResponse {
    pub error: String,
    pub status_code: StatusCode,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        (
            self.status_code,
            Json(json!(
                {
                    "error": self.error
                }
            )),
        )
            .into_response()
    }
}

pub trait CustomError {
    fn error(&self) -> ErrorResponse;
}
