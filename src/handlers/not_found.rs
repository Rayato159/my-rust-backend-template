use axum::{http::StatusCode, response::IntoResponse};

use crate::models::error::ErrorResponse;

pub async fn not_found() -> impl IntoResponse {
    ErrorResponse {
        error: "Endpoint not found".to_string(),
        status_code: StatusCode::NOT_FOUND,
    }
    .into_response()
}
