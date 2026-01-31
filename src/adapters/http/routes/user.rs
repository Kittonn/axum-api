use axum::{Extension, Json, Router, middleware, response::IntoResponse, routing::get};
use serde::Serialize;
use uuid::Uuid;

use crate::adapters::http::middlewares::auth_middleware::auth_middleware;
use crate::{adapters::http::app_state::AppState, domain::entities::user::User};

// adapters/http/routes/user.rs
pub fn user_routes() -> Router<AppState> {
    Router::new().route("/profile", get(get_profile))
}

#[derive(Debug, Serialize)]
pub struct UserProfileResponse {
    pub id: Uuid,
    pub email: String,
    pub name: String,
}
async fn get_profile(Extension(user): Extension<User>) -> impl IntoResponse {
    Json(UserProfileResponse {
        id: *user.id(),
        email: user.email().to_string(),
        name: user.name().to_string(),
    })
}
