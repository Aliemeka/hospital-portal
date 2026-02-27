use crate::admin::models::{CreateHospital, UpdateHospital};
use crate::admin::service;
use crate::app_state::SharedState;
use crate::auth::headers::ClaimsHeader;
use crate::errors::AppError;
use axum::extract::Path;
use axum::response::IntoResponse;
use axum::{Json, extract::State, http::StatusCode};
use serde_json::json;

pub async fn create_hospital_and_admin_handler(
    State(state): State<SharedState>,
    Json(data): Json<CreateHospital>,
) -> impl IntoResponse {
    let result = service::create_new_hospital_and_admin(state, data).await;
    match result {
        Ok(data) => Json(data).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

pub async fn get_hospital_info_handler(
    State(state): State<SharedState>,
    Path(hospital_id): Path<String>,
) -> impl IntoResponse {
    let hospital_id = match uuid::Uuid::parse_str(&hospital_id) {
        Ok(id) => id,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": format!("Invalid hospital_id: {}", e)})),
            )
                .into_response();
        }
    };
    let result = service::get_hospital_by_id(state, hospital_id).await;
    match result {
        Ok(data) => Json(data).into_response(),
        Err(e) => match e {
            AppError::NotFound(e) => {
                (StatusCode::NOT_FOUND, Json(json!({"error": e.to_string()}))).into_response()
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
                .into_response(),
        },
    }
}

pub async fn update_hospital_info_handler(
    State(state): State<SharedState>,
    claims: ClaimsHeader,
    Path(hospital_id): Path<String>,
    Json(data): Json<UpdateHospital>,
) -> impl IntoResponse {
    let hospital_id = match uuid::Uuid::parse_str(&hospital_id) {
        Ok(id) => id,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(json!({"error": format!("Invalid hospital_id: {}", e)})),
            )
                .into_response();
        }
    };
    let result = service::update_hospital_info(state, hospital_id, data, claims).await;
    match result {
        Ok(data) => Json(data).into_response(),
        Err(e) => match e {
            AppError::Unauthorized(e) => (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error": e.to_string()})),
            )
                .into_response(),
            AppError::NotFound(e) => {
                (StatusCode::NOT_FOUND, Json(json!({"error": e.to_string()}))).into_response()
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
                .into_response(),
        },
    }
}
