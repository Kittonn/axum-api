use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

pub trait TokenProvider: Send + Sync {
    fn generate_token(&self, user_id: &str, expiration: Duration) -> Result<String, String>;
    fn decode_token(&self, token: &str) -> Result<Claims, String>;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

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
    fn generate_token(&self, user_id: &str, expiration: Duration) -> Result<String, String> {
        let exp = Utc::now() + expiration;

        let claims = Claims {
            sub: user_id.to_owned(),
            exp: exp.timestamp() as usize,
        };

        encode(&Header::default(), &claims, &self.encoding).map_err(|e| e.to_string())
    }

    fn decode_token(&self, token: &str) -> Result<Claims, String> {
        match decode::<Claims>(token, &self.decoding, &Validation::default()) {
            Ok(data) => Ok(data.claims),
            Err(e) => Err(e.to_string()),
        }
    }
}
