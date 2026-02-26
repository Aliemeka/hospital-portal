use crate::app_state::SharedState;
use crate::billing::models::{CreateBillRequest, PayBillRequest};
use crate::billing::service::{issue_bill, pay_bill};
use crate::errors::AppError;
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;

pub async fn issue_bill_handler(
    State(state): State<SharedState>,
    Json(payload): Json<CreateBillRequest>,
) -> impl IntoResponse {
    match issue_bill(state, payload).await {
        Ok(bill) => (StatusCode::CREATED, Json(bill)).into_response(),
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

pub async fn pay_bill_handler(
    State(state): State<SharedState>,
    Json(payload): Json<PayBillRequest>,
) -> impl IntoResponse {
    match pay_bill(state, payload).await {
        Ok(bill) => (StatusCode::OK, Json(bill)).into_response(),
        Err(e) => match e {
            AppError::DatabaseError(_) => (
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Bill not found"})),
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
