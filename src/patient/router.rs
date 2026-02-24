use std::sync::Arc;

use crate::app_state::{AppState, SharedState};
use crate::patient::handler::{create_patient_handler, get_patients_handler};
use axum::Router;
use axum::routing::get;

pub fn patient_router(state: SharedState) -> Router<Arc<AppState>> {
    Router::new()
        .route("/", get(get_patients_handler).post(create_patient_handler))
        .with_state(state)
}
