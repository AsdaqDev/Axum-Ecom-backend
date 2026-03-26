use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

// 1️⃣ Define the error response body
#[derive(Serialize)]
struct ErrorRes {
    message: String,
}

// 2️⃣ Define your custom error enum
pub enum AppError {
    Unauthorized,
    NotFound,
    InternalError(String),
}

// 3️⃣ Implement IntoResponse so it can be returned from handlers
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "User not found".to_string()),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),
            AppError::InternalError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
        };

        // Build the response body
        let body = ErrorRes { message };

        // Convert to JSON response with status code
        (status, axum::Json(body)).into_response()
    }
}