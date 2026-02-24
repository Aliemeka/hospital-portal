use std::sync::Arc;

use axum::{
    Router,
    routing::{get, put},
};

use crate::{
    app_state::{AppState, SharedState},
    appointments::handlers::{
        create_appointment_handler, get_appointment_by_id_handler, get_appointments_handler,
        update_appointment_status_handler,
    },
};

pub fn appointments_router(state: SharedState) -> Router<Arc<AppState>> {
    Router::new()
        .route(
            "/",
            get(get_appointments_handler).post(create_appointment_handler),
        )
        .route("/<id>", get(get_appointment_by_id_handler))
        .route("/<id>/status", put(update_appointment_status_handler))
        .with_state(state)
}
