use crate::app_state::{AppState, SharedState};
use crate::doctor::handlers::{
    create_doctor_handler, get_all_doctors_handler, get_available_doctors_handler,
    get_doctor_by_id_handler,
};
use axum::Router;
use axum::routing::get;
use std::sync::Arc;

pub fn doctor_router(state: SharedState) -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/",
            get(get_all_doctors_handler).post(create_doctor_handler),
        )
        .route("/check/available", get(get_available_doctors_handler))
        .route("/<id>", get(get_doctor_by_id_handler))
        .with_state(state)
}
