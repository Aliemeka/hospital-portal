use crate::admin::handlers::{
    create_hospital_and_admin_handler, get_hospital_info_handler, update_hospital_info_handler,
};
use crate::app_state::{AppState, SharedState};
use axum::Router;
use axum::routing::{get, post};
use std::sync::Arc;

pub fn admin_router(state: SharedState) -> Router<Arc<AppState>> {
    Router::new()
        .route("/hospitals", post(create_hospital_and_admin_handler))
        .route(
            "/hospitals/{hospital_id}",
            get(get_hospital_info_handler).put(update_hospital_info_handler),
        )
        .with_state(state)
}
