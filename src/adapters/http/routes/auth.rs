use axum::{Json, Router, extract::State, routing::post};
use serde::{Deserialize, Serialize};

use crate::{
    adapters::http::{app_state::AppState, response::ApiSuccessResponse},
    application::app_error::AppError,
};

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
}

#[derive(Debug, Clone, Deserialize)]
pub struct RegisterRequest {
    name: String,
    email: String,
    password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CredentialsResponse {
    access_token: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<ApiSuccessResponse<CredentialsResponse>>, AppError> {
    let access_token = state
        .auth_use_case
        .register(payload.email, payload.password, payload.name)
        .await?;

    Ok(Json(ApiSuccessResponse::new(CredentialsResponse {
        access_token,
    })))
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<ApiSuccessResponse<CredentialsResponse>>, AppError> {
    let access_token = state
        .auth_use_case
        .login(payload.email, payload.password)
        .await?;

    Ok(Json(ApiSuccessResponse::new(CredentialsResponse {
        access_token,
    })))
}
