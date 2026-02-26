use crate::app_state::{AppState, SharedState};
use crate::auth::handlers::{get_user_info_handler, login_handler};
use axum::{
    Router,
    routing::{get, post},
};
use std::sync::Arc;

pub fn auth_router(state: SharedState) -> Router<Arc<AppState>> {
    Router::new()
        .route("/login", post(login_handler))
        .route("/me", get(get_user_info_handler))
        .with_state(state)
}
