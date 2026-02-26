use axum::extract::{Path, Query, State};
use std::collections::HashMap;
use uuid::Uuid;

use crate::appointments::models::CreateAppointmentRequest;
use crate::appointments::service;
use crate::{app_state::SharedState, errors::AppError};
use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

pub async fn get_appointments_handler(
    State(state): State<SharedState>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let patient_id = match params.get("patient_id").map(|s| Uuid::parse_str(s)) {
        Some(Ok(id)) => Some(id),
        Some(Err(e)) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"Invalid patient_id": e.to_string()})),
            )
                .into_response();
        }
        None => None,
    };

    let doctor_id = match params.get("doctor_id").map(|s| Uuid::parse_str(s)) {
        Some(Ok(id)) => Some(id),
        Some(Err(e)) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"Invalid doctor_id": e.to_string()})),
            )
                .into_response();
        }
        None => None,
    };

    match service::get_appointments(state, patient_id, doctor_id).await {
        Ok(appointments) => Json(appointments).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

pub async fn create_appointment_handler(
    State(state): State<SharedState>,
    Json(payload): Json<CreateAppointmentRequest>,
) -> impl IntoResponse {
    match service::create_appointment(state, payload).await {
        Ok(appointment) => (StatusCode::CREATED, Json(appointment)).into_response(),
        Err(e) => match e {
            AppError::DatabaseError(_) => (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Appointment not found"})),
            )
                .into_response(),
            AppError::ParsingError(e) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": e.to_string()})),
            )
                .into_response(),
            AppError::UnProcessableEntity { field, message } => (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({"error": format!("{}: {}", field, message)})),
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

pub async fn get_appointment_by_id_handler(
    State(state): State<SharedState>,
    Path(appointment_id): Path<String>,
) -> impl IntoResponse {
    match service::get_appointment_by_id(state, appointment_id).await {
        Ok(appointment) => Json(appointment).into_response(),
        Err(e) => match e {
            AppError::DatabaseError(_) => (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Appointment not found"})),
            )
                .into_response(),
            AppError::ParsingError(e) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": e.to_string()})),
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

pub async fn update_appointment_status_handler(
    State(state): State<SharedState>,
    Path(appointment_id): Path<String>,
    Query(params): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let status = match params.get("status") {
        Some(s) => s.clone(),
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": "Missing status parameter"})),
            )
                .into_response();
        }
    };

    match service::update_appointment_status(state, appointment_id, status).await {
        Ok(appointment) => Json(appointment).into_response(),
        Err(e) => match e {
            AppError::DatabaseError(_) => (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Appointment not found"})),
            )
                .into_response(),
            AppError::ParsingError(e) => (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": e.to_string()})),
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
