use crate::app_state::{AppState, SharedState};
use crate::config::AppConfig;
use crate::router::create_router;
use axum::serve;
use sqlx::PgPool;
use std::error::Error;
use tokio::net::TcpListener;

pub async fn start_app() -> Result<(), Box<dyn Error>> {
    let app_config = AppConfig::from_env().expect("Failed to load environment variables");

    let db_pool = PgPool::connect(&app_config.database_url)
        .await
        .expect("Failed to connect to the database");

    let app_state = SharedState::new(AppState::new(db_pool));
    let app = create_router(app_state);
    let local_server = format!("127.0.0.1:{}", &app_config.server_port);
    let listener = TcpListener::bind(&local_server).await.unwrap();

    serve(listener, app).await.unwrap();
    println!("Server is running on http://{local_server}");
    Ok(())
}
