use crate::patient::router::patient_router;
use crate::{
    app_state::SharedState, appointments::router::appointments_router,
    billing::router::billing_router, doctor::router::doctor_router,
};
use axum::{
    Router,
    http::StatusCode,
    response::{IntoResponse, Json},
    routing::get,
};
use serde_json::json;

pub fn create_router(state: SharedState) -> Router {
    Router::new()
        .nest("/patients", patient_router(state.clone()))
        .nest("/doctors", doctor_router(state.clone()))
        .nest("/appointments", appointments_router(state.clone()))
        .nest("/billing", billing_router(state.clone()))
        .route("/health", get(health_handler))
        .route("/", get(hello))
        .with_state(state)
}

async fn hello() -> &'static str {
    "Welcome to Mini Hospital Portal API!"
}

async fn health_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "status": "ok",
            "message": "Server is running"
        })),
    )
}
