use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use thiserror::Error;
use validator::ValidationErrors;

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

    #[error("Validation error: {0}")]
    ValidationError(#[from] ValidationErrors),

    #[error(transparent)]
    JsonRejection(#[from] JsonRejection),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, error_message) = match self {
            AppError::EmailAlreadyExists(_) => (StatusCode::CONFLICT, self.to_string()),
            AppError::UserNotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::InvalidToken => (StatusCode::UNAUTHORIZED, self.to_string()),
            AppError::ValidationError(errors) => (StatusCode::BAD_REQUEST, errors.to_string()),
            AppError::JsonRejection(rejection) => (StatusCode::BAD_REQUEST, rejection.to_string()),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
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
