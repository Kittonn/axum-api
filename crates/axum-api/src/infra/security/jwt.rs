use crate::application::app_error::AppError;
use chrono::{Duration, Utc};
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub trait TokenProvider: Send + Sync {
    fn generate_token(&self, user_id: &str, expiration: Duration) -> Result<String, AppError>;
    fn decode_token(&self, token: &str) -> Result<Claims, AppError>;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub jti: String,
    pub iat: i64,
    pub exp: i64,
}

#[derive(Clone)]
pub struct JwtTokenProvider {
    encoding: EncodingKey,
    decoding: DecodingKey,
}

impl JwtTokenProvider {
    pub fn new(secret: &str) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret.as_ref()),
            decoding: DecodingKey::from_secret(secret.as_ref()),
        }
    }
}

impl TokenProvider for JwtTokenProvider {
    fn generate_token(&self, user_id: &str, expiration: Duration) -> Result<String, AppError> {
        let now = Utc::now().timestamp();

        let claims = Claims {
            sub: user_id.to_owned(),
            jti: Uuid::new_v4().to_string(),
            iat: now,
            exp: now + expiration.num_seconds(),
        };

        let header = Header::new(Algorithm::HS256);

        encode(&header, &claims, &self.encoding)
            .map_err(|e| AppError::TokenGenerationFailed(e.to_string()))
    }

    fn decode_token(&self, token: &str) -> Result<Claims, AppError> {
        match decode::<Claims>(token, &self.decoding, &Validation::default()) {
            Ok(data) => Ok(data.claims),
            Err(e) => Err(AppError::TokenParsingFailed(e.to_string())),
        }
    }
}
