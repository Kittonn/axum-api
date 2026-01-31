use axum::{
    body::Body,
    extract::State,
    http::{Request, Response, StatusCode, header},
    middleware::Next,
};

use crate::adapters::http::app_state::AppState;

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, StatusCode> {
    let headers = req
        .headers()
        .get(header::AUTHORIZATION)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let auth_str = headers.to_str().map_err(|_| StatusCode::UNAUTHORIZED)?;

    let token = auth_str
        .strip_prefix("Bearer ")
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = state
        .token_provider
        .decode_token(token)
        .map_err(|_| StatusCode::UNAUTHORIZED)?;

    let current_user = state
        .user_use_case
        .get_user_by_id(&claims.sub)
        .await
        .ok_or(StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(current_user);

    Ok(next.run(req).await)
}
