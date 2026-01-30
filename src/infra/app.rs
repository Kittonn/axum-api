use axum::{
    Router,
    http::{
        Method,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
    routing::get,
};
use tower_http::cors::CorsLayer;

use crate::infra::setup::init_tracing;

pub fn create_app() -> Router {
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
        .route("/", get(|| async { "Hello, World!" }))
        .layer(cors)
}
