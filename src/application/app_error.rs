use axum::{
    Json,
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
};
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

    #[error("Validation failed")]
    ValidationError(Vec<String>),

    #[error(transparent)]
    JsonRejection(#[from] JsonRejection),
}

impl AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::EmailAlreadyExists(_) => StatusCode::CONFLICT,
            AppError::UserNotFound => StatusCode::NOT_FOUND,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::InvalidToken => StatusCode::UNAUTHORIZED,
            AppError::ValidationError(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::JsonRejection(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn details(&self) -> Option<Vec<String>> {
        match self {
            AppError::ValidationError(errors) => Some(errors.clone()),
            _ => None,
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();

        let body = serde_json::json!({
            "error": {
                "message": self.to_string(),
                "details": self.details(),
            }
        });

        (status, Json(body)).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
