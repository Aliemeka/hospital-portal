use crate::auth::models::Claims;
use crate::errors::AppError;
use crate::utils::get_secret_key;
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum_extra::TypedHeader;
use axum_extra::headers::{Authorization, authorization::Bearer};
use jsonwebtoken::{DecodingKey, Validation, decode};

impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, AppError> {
        // Extract the Authorization header
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| AppError::Unauthorized("Missing authorization header".to_string()))?;

        // Decode the JWT
        let token_data = decode::<Claims>(
            bearer.token(),
            &DecodingKey::from_secret(
                get_secret_key()
                    .expect("Failed to get secret key from config")
                    .as_bytes(),
            ),
            &Validation::default(),
        )
        .map_err(|_| AppError::Unauthorized("Invalid or expired token".to_string()))?;

        Ok(token_data.claims)
    }
}

pub type ClaimsHeader = Claims;
