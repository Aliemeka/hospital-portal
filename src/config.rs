use crate::errors::AppError;
use std::{env, fmt::Display, str::FromStr};

// pub const DEFAULT_REFERENCE_LENGTH: usize = 12;

pub const DEFAULT_APPOINTMENT_PRICE: f64 = 10000.00; // Default price for an appointment

pub struct AppConfig {
    pub database_url: String,
    pub server_port: u16,
    pub paystack_url: String, // For billing and payment processing
    pub paystack_secret_key: String,
    pub secret_key: String,
}

impl AppConfig {
    pub fn from_env() -> Result<Self, AppError> {
        let database_url = get_env_var("DATABASE_URL")?;
        let server_port = get_env_var("SERVER_PORT")?;
        let paystack_url = get_env_var("PAYSTACK_PAYMENT_URL")?;
        let paystack_secret_key = get_env_var("PAYSTACK_SECRET_KEY")?;
        let secret_key = get_env_var("SECRET_KEY")?;

        Ok(Self {
            database_url,
            server_port,
            paystack_url,
            paystack_secret_key,
            secret_key,
        })
    }
}

fn get_env_var<T: FromStr>(key: &str) -> Result<T, AppError>
where
    T::Err: Display,
{
    let value = env::var(key).map_err(|_| AppError::MissingEnvironmentVarible(key.to_string()))?;
    value.trim().parse::<T>().map_err(|e| {
        AppError::ParsingError(format!(
            "Failed to parse environment variable '{}': {}",
            key, e
        ))
    })
}
