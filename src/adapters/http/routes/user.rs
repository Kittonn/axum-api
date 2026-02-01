use axum::{Extension, Json, Router, routing::get};
use serde::Serialize;
use uuid::Uuid;

use crate::{
    adapters::http::{app_state::AppState, response::ApiSuccessResponse},
    application::app_error::AppError,
    domain::entities::user::User,
};

pub fn user_routes() -> Router<AppState> {
    Router::new().route("/profile", get(get_profile))
}

#[derive(Debug, Serialize)]
pub struct UserProfileResponse {
    pub id: Uuid,
    pub email: String,
    pub name: String,
}
async fn get_profile(
    Extension(user): Extension<User>,
) -> Result<Json<ApiSuccessResponse<UserProfileResponse>>, AppError> {
    Ok(Json(ApiSuccessResponse::new(UserProfileResponse {
        id: *user.id(),
        email: user.email().to_string(),
        name: user.name().to_string(),
    })))
}
