use crate::{config::AppConfig, errors::AppError};
use rand::{RngExt, distr::Alphanumeric, rng};

pub fn create_random_string(length: usize) -> String {
    let mut rng = rng();
    (0..length)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect()
}

pub const VALID_DAYS: [&str; 7] = [
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
    "Sunday",
];

pub fn is_valid_day(day: &str) -> bool {
    VALID_DAYS.contains(&day)
}

pub fn get_paystack_config() -> Result<(String, String), AppError> {
    let app_config = AppConfig::from_env()?;
    Ok((app_config.paystack_url, app_config.paystack_secret_key))
}
