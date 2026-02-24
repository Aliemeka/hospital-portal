use chrono::Utc;
use rand::seq::IteratorRandom;
use sqlx::QueryBuilder;
use uuid::Uuid;

use crate::appointments::models::{
    Appointment, AppointmentList, AppointmentStatus, CreateAppointmentRequest,
};
use crate::doctor::service::get_available_doctors;
use crate::{app_state::SharedState, config::DEFAULT_APPOINTMENT_PRICE, errors::AppError};

// Get all, optionally filter by eithor or both patient id or doctor id
pub async fn get_appointments(
    state: SharedState,
    patient_id: Option<Uuid>,
    doctor_id: Option<Uuid>,
) -> Result<AppointmentList, AppError> {
    let mut builder = QueryBuilder::new(
        "SELECT id, patient_id, doctor_id, purpose, time, status, price FROM appointments",
    );

    let mut where_added = false;

    if let Some(pid) = patient_id {
        builder.push(" WHERE patient_id = ");
        builder.push_bind(pid);
        where_added = true;
    }

    if let Some(did) = doctor_id {
        if where_added {
            builder.push(" AND doctor_id = ");
        } else {
            builder.push(" WHERE doctor_id = ");
        }
        builder.push_bind(did);
    }

    let appointments = builder
        .build_query_as::<Appointment>()
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(AppointmentList { appointments })
}

// Get a specific appointment by id
pub async fn get_appointment_by_id(
    state: SharedState,
    appointment_id: String,
) -> Result<Appointment, AppError> {
    let appointment_id =
        Uuid::parse_str(&appointment_id).map_err(|e| AppError::ParsingError(e.to_string()))?;
    let appointment = sqlx::query_as::<_, Appointment>(
        "SELECT id, patient_id, doctor_id, purpose, time, status, price FROM appointments WHERE id = $1",
    )
    .bind(appointment_id)
    .fetch_one(&state.db_pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(appointment)
}

// Create an appointment
pub async fn create_appointment(
    state: SharedState,
    payload: CreateAppointmentRequest,
) -> Result<Appointment, AppError> {
    let available = get_available_doctors(state.clone(), payload.day).await?;

    let doctor = available
        .doctors
        .into_iter()
        .choose(&mut rand::rng())
        .ok_or_else(|| {
            AppError::NotFound("No doctors available on the requested day".to_string())
        })?;

    let time = chrono::DateTime::parse_from_rfc3339(&payload.time)
        .map(|dt| dt.with_timezone(&Utc))
        .map_err(|e| AppError::ParsingError(e.to_string()))?;

    let appointment = Appointment::new(
        payload.patient_id,
        doctor.id,
        payload.purpose,
        time,
        DEFAULT_APPOINTMENT_PRICE,
    );

    sqlx::query(
        "INSERT INTO appointments (id, patient_id, doctor_id, purpose, time, status, price) VALUES ($1, $2, $3, $4, $5, $6, $7)",
    )
    .bind(appointment.id)
    .bind(appointment.patient_id)
    .bind(appointment.doctor_id)
    .bind(&appointment.purpose)
    .bind(appointment.time)
    .bind(&appointment.status)
    .bind(appointment.price)
    .execute(&state.db_pool)
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(appointment)
}

// Update appointment status
pub async fn update_appointment_status(
    state: SharedState,
    appointment_id: String,
    status: String,
) -> Result<Appointment, AppError> {
    let status = match status.as_str() {
        "Scheduled" => AppointmentStatus::Scheduled,
        "Done" => AppointmentStatus::Done,
        "Cancelled" => AppointmentStatus::Cancelled,
        _ => {
            return Err(AppError::ParsingError(format!(
                "Invalid status value: {status}",
            )));
        }
    };
    let id = Uuid::parse_str(&appointment_id).map_err(|e| AppError::ParsingError(e.to_string()))?;
    sqlx::query("UPDATE appointments SET status = $1 WHERE id = $2")
        .bind(&status)
        .bind(id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    get_appointment_by_id(state, appointment_id).await
}
