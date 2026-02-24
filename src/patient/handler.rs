use crate::app_state::SharedState;
use crate::patient::models::CreatePatient;
use crate::patient::service::create_patient;
use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde_json::json;

pub async fn get_patients_handler(State(state): State<SharedState>) -> impl IntoResponse {
    match crate::patient::service::get_patients(state).await {
        Ok(patients) => (StatusCode::OK, Json(patients)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

pub async fn create_patient_handler(
    State(state): State<SharedState>,
    Json(payload): Json<CreatePatient>,
) -> impl IntoResponse {
    match create_patient(state, payload).await {
        Ok(patient) => (StatusCode::CREATED, Json(patient)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}
