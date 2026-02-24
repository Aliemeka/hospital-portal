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
