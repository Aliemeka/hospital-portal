use crate::app_state::SharedState;
use crate::doctor::models::{CreateDoctor, Doctor, DoctorList};
use crate::errors::AppError;
use crate::utils::is_valid_day;

// Get all doctors
pub async fn get_all_doctors(state: SharedState) -> Result<DoctorList, AppError> {
    let doctors = sqlx::query_as::<_, Doctor>("SELECT * FROM doctors")
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(DoctorList { doctors })
}

// Get doctor by ID
pub async fn get_doctor_by_id(state: SharedState, doctor_id: String) -> Result<Doctor, AppError> {
    let id =
        uuid::Uuid::parse_str(&doctor_id).map_err(|e| AppError::ParsingError(e.to_string()))?;
    let doctor = sqlx::query_as::<_, Doctor>("SELECT * FROM doctors WHERE id = $1")
        .bind(id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(doctor)
}

// Create a new doctor
pub async fn create_doctor(
    state: SharedState,
    doctor_data: CreateDoctor,
) -> Result<Doctor, AppError> {
    let doctor = Doctor::new(doctor_data);
    sqlx::query("INSERT INTO doctors (id, name, specialization, visiting_hours, available_days) VALUES ($1, $2, $3, $4, $5)")
        .bind(doctor.id)
        .bind(&doctor.name)
        .bind(&doctor.specialization)
        .bind(&doctor.visiting_hours)
        .bind(&doctor.available_days as &[String])
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(doctor)
}

// Get doctors available on a specific day and time
// This will be used for appointment scheduling to show available doctors based on the selected day and time
pub async fn get_available_doctors(
    state: SharedState,
    day: String,
) -> Result<DoctorList, AppError> {
    if !is_valid_day(&day) {
        return Err(AppError::ParsingError("Invalid day format".to_string()));
    }
    let doctors =
        sqlx::query_as::<_, Doctor>("SELECT * FROM doctors WHERE $1 = ANY(available_days)")
            .bind(day)
            .fetch_all(&state.db_pool)
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(DoctorList { doctors })
}

// Check if a doctor is available on a specific day and time
// Will use this for appointment scheduling
pub async fn is_doctor_available(
    state: SharedState,
    doctor_id: String,
    day: String,
) -> Result<bool, AppError> {
    if !is_valid_day(&day) {
        return Err(AppError::ParsingError("Invalid day format".to_string()));
    }
    let id =
        uuid::Uuid::parse_str(&doctor_id).map_err(|e| AppError::ParsingError(e.to_string()))?;
    let doctor = sqlx::query_as::<_, Doctor>("SELECT * FROM doctors WHERE id = $1")
        .bind(id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(doctor.available_days.contains(&day))
}
