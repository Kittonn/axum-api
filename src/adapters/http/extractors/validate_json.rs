use axum::{
    Json,
    extract::{FromRequest, Request},
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::application::app_error::AppError;

pub struct ValidateJson<T>(pub T);

impl<T, S> FromRequest<S> for ValidateJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(payload) = Json::<T>::from_request(req, state).await?;
        payload.validate().map_err(AppError::ValidationError)?;
        Ok(ValidateJson(payload))
    }
}
