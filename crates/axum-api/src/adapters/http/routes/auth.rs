use crate::adapters::http::{
    dto::validators::validate_uuid::validate_uuid, extractors::validate_json::ValidateJson,
};
use axum::{Extension, Json, Router, extract::State, middleware, routing::post};

use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    adapters::http::{
        app_state::AppState, middlewares::auth_middleware::auth_middleware,
        response::ApiSuccessResponse,
    },
    application::app_error::AppError,
    infra::security::jwt::Claims,
};

pub fn auth_routes(state: AppState) -> Router<AppState> {
    let public_routes = Router::new()
        .route("/login", post(login))
        .route("/register", post(register))
        .route("/refresh", post(refresh));

    let protected_routes =
        Router::new()
            .route("/logout", post(logout))
            .route_layer(middleware::from_fn_with_state(
                state.clone(),
                auth_middleware,
            ));

    public_routes.merge(protected_routes)
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct RegisterRequest {
    #[validate(length(min = 1, max = 100, message = "Name must be 1-100 characters"))]
    name: String,

    #[validate(email(message = "Invalid email format"))]
    email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    password: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct CredentialsResponse {
    access_token: String,
    refresh_token: String,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct LoginRequest {
    #[validate(email(message = "Invalid email format"))]
    email: String,

    #[validate(length(min = 8, message = "Password must be at least 8 characters"))]
    password: String,
}

#[derive(Debug, Clone, Deserialize, Validate)]
pub struct RefreshRequest {
    #[validate(custom(function = "validate_uuid"))]
    refresh_token: String,
}

async fn register(
    State(state): State<AppState>,
    ValidateJson(payload): ValidateJson<RegisterRequest>,
) -> Result<Json<ApiSuccessResponse<CredentialsResponse>>, AppError> {
    let (access_token, refresh_token) = state
        .auth_use_case
        .register(payload.email, payload.password, payload.name)
        .await?;

    Ok(Json(ApiSuccessResponse::new(CredentialsResponse {
        access_token,
        refresh_token,
    })))
}

async fn login(
    State(state): State<AppState>,
    ValidateJson(payload): ValidateJson<LoginRequest>,
) -> Result<Json<ApiSuccessResponse<CredentialsResponse>>, AppError> {
    let (access_token, refresh_token) = state
        .auth_use_case
        .login(payload.email, payload.password)
        .await?;

    Ok(Json(ApiSuccessResponse::new(CredentialsResponse {
        access_token,
        refresh_token,
    })))
}

async fn refresh(
    State(state): State<AppState>,
    ValidateJson(payload): ValidateJson<RefreshRequest>,
) -> Result<Json<ApiSuccessResponse<CredentialsResponse>>, AppError> {
    let (access_token, refresh_token) = state
        .auth_use_case
        .refresh_token(&payload.refresh_token)
        .await?;

    Ok(Json(ApiSuccessResponse::new(CredentialsResponse {
        access_token,
        refresh_token,
    })))
}

async fn logout(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ApiSuccessResponse<()>>, AppError> {
    state
        .auth_use_case
        .revoke_token(&claims.jti, claims.exp)
        .await?;

    Ok(Json(ApiSuccessResponse::new(())))
}
