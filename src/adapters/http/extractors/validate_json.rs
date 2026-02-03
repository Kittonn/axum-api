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
        match Json::<T>::from_request(req, state).await {
            Ok(Json(payload)) => {
                if let Err(e) = payload.validate() {
                    let error_messages: Vec<String> = e
                        .field_errors()
                        .into_iter()
                        .flat_map(|(field, field_errors)| {
                            field_errors.iter().map(move |error| {
                                if let Some(message) = &error.message {
                                    message.to_string()
                                } else {
                                    format!("{} is invalid", field)
                                }
                            })
                        })
                        .collect();

                    return Err(AppError::ValidationError(error_messages));
                }

                Ok(ValidateJson(payload))
            }
            Err(rejection) => Err(AppError::JsonRejection(rejection)),
        }
    }
}
