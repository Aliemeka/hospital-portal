use crate::admin::handlers::create_hospital_and_admin_handler;
use crate::app_state::{AppState, SharedState};
use axum::Router;
use axum::routing::post;
use std::sync::Arc;

pub fn admin_router(state: SharedState) -> Router<Arc<AppState>> {
    Router::new()
        .route("/new_hospital", post(create_hospital_and_admin_handler))
        .with_state(state)
}
