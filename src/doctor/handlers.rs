use crate::app_state::SharedState;
use crate::doctor::models::CreateDoctor;
use crate::doctor::service;
use crate::errors::AppError;
use axum::{
    Json,
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use std::collections::HashMap;

// Get all doctors handler
pub async fn get_all_doctors_handler(State(state): State<SharedState>) -> impl IntoResponse {
    match service::get_all_doctors(state).await {
        Ok(doctor_list) => Json(doctor_list).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

// Get doctor by ID handler
pub async fn get_doctor_by_id_handler(
    State(state): State<SharedState>,
    axum::extract::Path(doctor_id): axum::extract::Path<String>,
) -> impl IntoResponse {
    match service::get_doctor_by_id(state, doctor_id).await {
        Ok(doctor) => Json(doctor).into_response(),
        Err(e) => match e {
            AppError::DatabaseError(_) => (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Doctor not found"})),
            )
                .into_response(),
            AppError::ParsingError(_) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Invalid doctor ID format"})),
            )
                .into_response(),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
                .into_response(),
        },
    }
}

// Create a new doctor handler
pub async fn create_doctor_handler(
    State(state): State<SharedState>,
    Json(payload): Json<CreateDoctor>,
) -> impl IntoResponse {
    match service::create_doctor(state, payload).await {
        Ok(doctor) => (StatusCode::CREATED, Json(doctor)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

// Get doctors available on a specific day and time handler
pub async fn get_available_doctors_handler(
    State(state): State<SharedState>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let day = match params.get("day") {
        Some(d) => d.clone(),
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Missing 'day' query parameter"})),
            )
                .into_response();
        }
    };

    match service::get_available_doctors(state, day).await {
        Ok(doctor_list) => Json(doctor_list).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}
