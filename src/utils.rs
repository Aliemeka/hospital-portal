use crate::{config::AppConfig, errors::AppError};
use bcrypt::{BcryptError, DEFAULT_COST, hash, verify};
use chrono::{DateTime, Datelike, Duration, NaiveDate, Utc, Weekday};
use rand::{RngExt, distr::Alphanumeric, rng};
use serde_json::map;

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

fn parse_weekday(day: &str) -> Option<Weekday> {
    match day {
        "Monday" => Some(Weekday::Mon),
        "Tuesday" => Some(Weekday::Tue),
        "Wednesday" => Some(Weekday::Wed),
        "Thursday" => Some(Weekday::Thu),
        "Friday" => Some(Weekday::Fri),
        "Saturday" => Some(Weekday::Sat),
        "Sunday" => Some(Weekday::Sun),
        _ => None,
    }
}

fn next_occurrence_of_weekday(weekday: Weekday) -> NaiveDate {
    let today = Utc::now().date_naive();
    let days_until = (weekday.num_days_from_monday() as i64
        - today.weekday().num_days_from_monday() as i64)
        .rem_euclid(7);

    // if today is that day, schedule for next week instead
    let days_until = if days_until == 0 { 7 } else { days_until };

    today + Duration::days(days_until)
}

pub fn normalize_timezone(input: &str) -> String {
    input
        .trim()
        .replace("WAT", "+01:00")
        .replace("CAT", "+02:00")
        .replace("EAT", "+03:00")
        .replace("GMT", "+00:00")
}

pub fn combine_day_and_time(day: &str, time_str: &str) -> Result<DateTime<Utc>, AppError> {
    let weekday = parse_weekday(day).ok_or_else(|| AppError::UnProcessableEntity {
        field: "day".to_string(),
        message: "Invalid day provided".to_string(),
    })?;

    let date = next_occurrence_of_weekday(weekday);
    let normalized = normalize_timezone(time_str);

    // Build a full RFC3339 string and parse it
    let full_datetime_str = format!("{}T{}", date, normalized);

    let parsed = DateTime::parse_from_rfc3339(&full_datetime_str).map_err(|e| {
        AppError::UnProcessableEntity {
            field: "time".to_string(),
            message: format!(
                "Could not parse time: {} {}",
                full_datetime_str,
                e.to_string()
            ),
        }
    })?;

    Ok(parsed.with_timezone(&Utc))
}

pub fn hash_password(password: &str) -> Result<String, AppError> {
    let hashed =
        hash(password, DEFAULT_COST).map_err(|e| AppError::InternalServerError(e.to_string()))?;
    Ok(hashed)
}

pub fn verify_password(password: &str, hashed: &str) -> Result<bool, AppError> {
    let result =
        verify(password, hashed).map_err(|e| AppError::InternalServerError(e.to_string()))?;
    Ok(result)
}

pub fn get_secret_key() -> Result<String, AppError> {
    let app_config = AppConfig::from_env()?;
    Ok(app_config.secret_key)
}
