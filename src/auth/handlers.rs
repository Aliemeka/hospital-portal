use crate::{
    app_state::SharedState, auth::headers::ClaimsHeader, auth::models::LoginRequest, auth::service,
    errors::AppError,
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde_json::json;

pub async fn login_handler(
    State(state): State<SharedState>,
    Json(data): Json<LoginRequest>,
) -> impl IntoResponse {
    let result = service::verify_login(state, data).await;
    match result {
        Ok(response) => Json(response).into_response(),
        Err(e) => (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

pub async fn get_user_info_handler(
    State(state): State<SharedState>,
    claims: ClaimsHeader,
) -> impl IntoResponse {
    let result = service::get_user_from_claims(state, claims).await;
    match result {
        Ok(user) => Json(user).into_response(),
        Err(e) => match e {
            AppError::Unauthorized(e) => (
                StatusCode::UNAUTHORIZED,
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
