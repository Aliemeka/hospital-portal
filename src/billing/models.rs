use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

use crate::utils::create_random_string;

#[derive(Serialize, FromRow)]
pub struct Bill {
    pub id: Uuid,
    pub reference: String, // Unique reference for the bill, can be generated using a utility function
    pub appointment_id: Uuid,
    pub amount: f64,
    pub currency: String,   // e.g., "USD", "NGN"
    pub status: BillStatus, // e.g., "pending", "paid", "cancelled"
}

#[derive(Serialize, Deserialize, Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "PascalCase")]
pub enum BillStatus {
    Pending,
    Paid,
    Cancelled,
}

#[derive(Deserialize)]
pub struct CreateBillRequest {
    pub appointment_id: String,
    pub amount: Option<f64>, // Optional, can be calculated based on appointment details
    pub currency: Option<String>, // Optional, default to a specific currency if not provided
}

#[derive(Deserialize)]
pub struct PayBillRequest {
    pub bill_id: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct PayStackRequest {
    pub email: String,
    pub amount: u64,
    pub reference: String,
    pub callback_url: String,
}

#[derive(Serialize)]
pub struct AuthorizationResponse {
    pub authorization_url: String,
    pub reference: String,
}

impl Bill {
    pub fn new(appointment_id: Uuid, amount: f64, currency: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            reference: create_random_string(10), // Generate a unique reference
            appointment_id,
            amount,
            currency: currency.unwrap_or_else(|| "NGN".to_string()), // Default to "NGN" if not provided
            status: BillStatus::Pending,
        }
    }
}
