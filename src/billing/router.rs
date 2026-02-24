use crate::app_state::{AppState, SharedState};
use crate::billing::handlers::{issue_bill_handler, pay_bill_handler};
use axum::Router;
use axum::routing::post;
use std::sync::Arc;

pub fn billing_router(state: SharedState) -> Router<Arc<AppState>> {
    Router::new()
        .route("/issue", post(issue_bill_handler))
        .route("/pay", post(pay_bill_handler))
        .with_state(state)
}
