use crate::admin::models::CreateHospital;
use crate::admin::service;
use crate::app_state::SharedState;
use axum::response::IntoResponse;
use axum::{Json, extract::State};

pub async fn create_hospital_and_admin_handler(
    State(state): State<SharedState>,
    Json(data): Json<CreateHospital>,
) -> impl IntoResponse {
    let result = service::create_new_hospital_and_admin(state, data).await;
    match result {
        Ok(data) => Json(data).into_response(),
        Err(e) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}
