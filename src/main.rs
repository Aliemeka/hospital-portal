mod admin;
mod app;
mod app_state;
mod appointments;
mod auth;
mod billing;
mod config;
mod doctor;
mod errors;
mod patient;
mod router;
mod utils;

use app::start_app;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    start_app().await.unwrap();
}
