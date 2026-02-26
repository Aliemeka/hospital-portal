use crate::admin::models::{User, UserRole};
use crate::utils::get_secret_key;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid, // user id
    pub hospital_id: Uuid,
    pub role: UserRole,
    pub exp: usize, // expiry
}

impl Claims {
    pub fn new(user: User) -> Self {
        Claims {
            sub: user.id,
            hospital_id: user.hospital_id,
            role: user.role,
            exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
        }
    }

    pub fn generate_token(&self) -> String {
        let header = Header::new(Algorithm::HS256);

        let secret_key = get_secret_key().expect("Failed to get secret key from config");

        let token = encode(
            &header,
            self,
            &EncodingKey::from_secret(&secret_key.as_bytes()),
        )
        .expect("Failed to encode JWT");

        token
    }
}
