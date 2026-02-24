use crate::app_state::SharedState;
use crate::appointments::service::get_appointment_by_id;
use crate::billing::models::{
    AuthorizationResponse, Bill, BillStatus, CreateBillRequest, PayBillRequest, PayStackRequest,
};
use crate::{errors::AppError, utils::get_paystack_config};
use reqwest::Client;
use serde_json::Value;
use uuid::Uuid;

pub async fn issue_bill(state: SharedState, payload: CreateBillRequest) -> Result<Bill, AppError> {
    let app_id = Uuid::parse_str(&payload.appointment_id)
        .map_err(|e| AppError::ParsingError(e.to_string()))?;
    match get_appointment_by_id(state.clone(), payload.appointment_id.clone()).await {
        Ok(appointment) => {
            let amount = payload.amount.unwrap_or(appointment.price);
            let bill = Bill::new(app_id, amount, payload.currency);
            sqlx::query("INSERT INTO bills (id, appointment_id, amount, currency, status) VALUES ($1, $2, $3, $4, $5)")
                    .bind(&bill.id)
                    .bind(&bill.appointment_id)
                    .bind(&bill.amount)
                    .bind(&bill.currency)
                    .bind(&bill.status)
                    .execute(&state.db_pool)
                    .await
                    .map_err(|e| AppError::DatabaseError(e.to_string()))?;
            Ok(bill)
        }
        Err(e) => {
            return Err(AppError::DatabaseError(format!(
                "Appointment not found: {}",
                e.to_string()
            )));
        }
    }
}

async fn get_bill_by_id(state: SharedState, bill_id: String) -> Result<Bill, AppError> {
    let id = Uuid::parse_str(&bill_id).map_err(|e| AppError::ParsingError(e.to_string()))?;
    let bill = sqlx::query_as::<_, Bill>("SELECT * FROM bills WHERE id = $1")
        .bind(id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(bill)
}

pub async fn pay_bill(
    state: SharedState,
    payload: PayBillRequest,
) -> Result<AuthorizationResponse, AppError> {
    let bill = get_bill_by_id(state.clone(), payload.bill_id.clone()).await?;

    let (paystack_url, paystack_secret_key) = get_paystack_config()?;

    match bill.status {
        BillStatus::Paid => {
            return Err(AppError::DatabaseError(
                "Bill has already been paid".to_string(),
            ));
        }
        _ => {}
    }
    let reference = &bill.reference;
    let paystack_request = PayStackRequest {
        email: payload.email,
        amount: (bill.amount * 100.0) as u64, // Convert to kobo
        reference: reference.clone(),
        callback_url: "https://yourapp.com/payment/callback".to_string(),
    };
    let client = Client::new();
    let response = client
        .post(&format!("{}/transaction/initialize", &paystack_url))
        .header("Authorization", format!("Bearer {}", &paystack_secret_key))
        .json(&paystack_request)
        .send()
        .await
        .map_err(|e| {
            AppError::InternalServerError(format!("Failed to send request to Paystack: {}", e))
        })?;

    if response.status().is_success() {
        let response_json: Value = response.json().await.map_err(|e| {
            AppError::ParsingError(format!("Failed to parse Paystack response: {}", e))
        })?;
        let authorization_url = response_json["data"]["authorization_url"]
            .as_str()
            .ok_or_else(|| {
                AppError::ParsingError(
                    "Missing 'authorization_url' in Paystack response".to_string(),
                )
            })?
            .to_string();

        Ok(AuthorizationResponse {
            authorization_url,
            reference: reference.clone(),
        })
    } else {
        let error_message = response
            .text()
            .await
            .unwrap_or_else(|_| "Unknown error".to_string());
        Err(AppError::InternalServerError(format!(
            "Paystack API returned an error: {}",
            error_message
        )))
    }
}
