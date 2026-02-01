use axum::{Json, Router, extract::State, response::IntoResponse, routing::post};
use serde::{Deserialize, Serialize};

use crate::adapters::http::app_state::AppState;

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
pub struct RegisterResponse {
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
) -> impl IntoResponse {
    match state
        .auth_use_case
        .register(payload.email, payload.password, payload.name)
        .await
    {
        Ok(access_token) => Json(RegisterResponse { access_token }),
        Err(err) => Json(RegisterResponse { access_token: err }),
    }
}

async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    match state
        .auth_use_case
        .login(payload.email, payload.password)
        .await
    {
        Ok(access_token) => Json(RegisterResponse { access_token }),
        Err(err) => Json(RegisterResponse { access_token: err }),
    }
}
