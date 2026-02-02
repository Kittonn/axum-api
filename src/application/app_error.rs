use axum::response::{IntoResponse, Response};
use thiserror::Error;

use crate::domain::repositories::error::RepositoryError;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Email {0} already exists")]
    EmailAlreadyExists(String),

    #[error("User not found")]
    UserNotFound,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Invalid token")]
    InvalidToken,

    #[error("Token generation failed: {0}")]
    TokenGenerationFailed(String),

    #[error("Token is invalid: {0}")]
    TokenParsingFailed(String),

    #[error("Password hashing failed: {0}")]
    PasswordHashingFailed(String),

    #[error("Password verification failed: {0}")]
    PasswordVerificationFailed(String),

    #[error(transparent)]
    RepositoryError(#[from] RepositoryError),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, error_message) = match self {
            AppError::EmailAlreadyExists(_) => (axum::http::StatusCode::CONFLICT, self.to_string()),
            AppError::UserNotFound => (axum::http::StatusCode::NOT_FOUND, self.to_string()),
            AppError::Unauthorized => (axum::http::StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::InvalidToken => (axum::http::StatusCode::UNAUTHORIZED, self.to_string()),
            _ => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                "Internal Server Error".to_string(),
            ),
        };

        let body = serde_json::json!({
            "error": error_message,
        });

        (status_code, axum::Json(body)).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
