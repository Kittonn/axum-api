use axum::{Router, routing::post};

use crate::adapters::http::app_state::AppState;

pub fn auth_routes() -> Router<AppState> {
    Router::new()
        .route("/login", post(|| async { "Login" }))
        .route("/register", post(|| async { "Register" }))
}
