use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use uuid::Uuid;

// patient id, doctor id, time, purpose, status, price,
#[derive(Serialize, FromRow)]
pub struct Appointment {
    pub id: Uuid,
    pub patient_id: Uuid,
    pub doctor_id: Uuid,
    pub purpose: String,
    pub time: DateTime<Utc>,
    pub status: AppointmentStatus,
    pub price: f64,
}

#[derive(Serialize, Deserialize, Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "PascalCase")]
pub enum AppointmentStatus {
    Scheduled,
    Done,
    Cancelled,
}

#[derive(Deserialize)]
pub struct CreateAppointmentRequest {
    pub patient_id: Uuid,
    pub day: String,
    pub time: String,
    pub purpose: String,
}

#[derive(Serialize)]
pub struct AppointmentList {
    pub appointments: Vec<Appointment>,
}

impl Appointment {
    pub fn new(
        patient_id: Uuid,
        doctor_id: Uuid,
        purpose: String,
        time: DateTime<Utc>,
        price: f64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            patient_id,
            doctor_id,
            purpose,
            time,
            status: AppointmentStatus::Scheduled,
            price,
        }
    }
}
