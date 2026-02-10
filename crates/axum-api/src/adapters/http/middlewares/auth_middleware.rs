use axum::{
    body::Body,
    extract::State,
    http::{Request, Response, header},
    middleware::Next,
};

use crate::{adapters::http::app_state::AppState, application::app_error::AppError};

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, AppError> {
    let headers = req
        .headers()
        .get(header::AUTHORIZATION)
        .ok_or(AppError::Unauthorized)?;

    let auth_str = headers.to_str().map_err(|_| AppError::Unauthorized)?;

    let token = auth_str
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized)?;
    let claims = state
        .token_provider
        .decode_token(token)
        .map_err(|_| AppError::Unauthorized)?;

    if state
        .auth_use_case
        .is_blacklisted(&claims.jti)
        .await
        .unwrap_or(true)
    {
        return Err(AppError::Unauthorized);
    }

    let current_user = state
        .user_use_case
        .get_user_by_id(&claims.sub)
        .await
        .map_err(|_| AppError::Unauthorized)?
        .ok_or(AppError::Unauthorized)?;

    req.extensions_mut().insert(claims.clone());
    req.extensions_mut().insert(current_user);

    Ok(next.run(req).await)
}
