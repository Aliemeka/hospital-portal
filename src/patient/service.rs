use crate::app_state::SharedState;
use crate::errors::AppError;
use crate::patient::models::{CreatePatient, Patient, PatientList};

pub async fn get_patients(state: SharedState) -> Result<PatientList, AppError> {
    let query = "SELECT * FROM patients";
    let patients = sqlx::query_as::<_, Patient>(query)
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(PatientList { patients })
}

pub async fn create_patient(
    state: SharedState,
    patient_data: CreatePatient,
) -> Result<Patient, AppError> {
    let patient = Patient::new(patient_data);
    let query = "INSERT INTO patients (id, name, age, card_id, gender) VALUES ($1, $2, $3, $4, $5)";
    sqlx::query(query)
        .bind(patient.id)
        .bind(&patient.name)
        .bind(patient.age)
        .bind(&patient.card_id)
        .bind(&patient.gender)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
    Ok(patient)
}
