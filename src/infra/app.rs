use axum::{
    Router,
    http::{
        Method,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
};
use tower_http::cors::CorsLayer;

use crate::{
    adapters::http::{app_state::AppState, routes::auth::auth_routes},
    infra::setup::init_tracing,
};

pub fn create_app(app_state: AppState) -> Router {
    init_tracing();

    let cors = CorsLayer::new()
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
        ])
        .allow_credentials(true)
        .allow_headers([CONTENT_TYPE, AUTHORIZATION]);

    Router::new()
        .nest("/auth", auth_routes().with_state(app_state.clone()))
        .layer(cors)
}
