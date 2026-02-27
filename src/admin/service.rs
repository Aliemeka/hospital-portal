use crate::admin::models::{
    Hospital, HospitalData, HospitalWithAdminEmail, UpdateHospital, User, UserRole,
};
use crate::auth::headers::ClaimsHeader;
use crate::errors::AppError;
use crate::{admin::models::CreateHospital, app_state::SharedState};
use uuid::Uuid;

pub async fn create_new_hospital_and_admin(
    state: SharedState,
    data: CreateHospital,
) -> Result<HospitalWithAdminEmail, AppError> {
    let data = HospitalData::new(data);
    let hospital = data.hospital;
    let admin = data.admin;
    sqlx::query("INSERT INTO hospitals (id, name, address, phone) VALUES ($1, $2, $3, $4)")
        .bind(hospital.id)
        .bind(&hospital.name)
        .bind(&hospital.address)
        .bind(&hospital.phone)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    sqlx::query("INSERT INTO users (id, name, email, password_hash, role, hospital_id) VALUES ($1, $2, $3, $4, $5, $6)")
        .bind(admin.id)
        .bind(&admin.name)
        .bind(&admin.email)
        .bind(&admin.password_hash)
        .bind(&admin.role)
        .bind(&admin.hospital_id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(HospitalWithAdminEmail {
        hospital,
        admin_email: admin.email,
    })
}

pub async fn get_user_by_email(state: SharedState, email: String) -> Result<User, AppError> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
        .bind(email)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(user)
}

pub async fn get_user_by_id(state: SharedState, user_id: Uuid) -> Result<User, AppError> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(user_id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(user)
}

pub async fn get_hospital_by_id(
    state: SharedState,
    hospital_id: Uuid,
) -> Result<Hospital, AppError> {
    let hospital = sqlx::query_as::<_, Hospital>("SELECT * FROM hospitals WHERE id = $1")
        .bind(hospital_id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(hospital)
}

pub async fn update_hospital_info(
    state: SharedState,
    hospital_id: Uuid,
    update_data: UpdateHospital,
    claim: ClaimsHeader,
) -> Result<Hospital, AppError> {
    match claim.role {
        UserRole::Admin => (),
        _ => {
            return Err(AppError::Unauthorized(
                "Only admin users can update hospital information".to_string(),
            ));
        }
    }
    if claim.hospital_id != hospital_id {
        return Err(AppError::Unauthorized(
            "You cannot update information for a hospital you do not belong to".to_string(),
        ));
    }

    let hospital = sqlx::query_as::<_, Hospital>("UPDATE hospitals SET name = COALESCE($1, name), address = COALESCE($2, address), phone = COALESCE($3, phone) WHERE id = $4 RETURNING *")
        .bind(update_data.name)
        .bind(update_data.address)
        .bind(update_data.phone)
        .bind(hospital_id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                AppError::NotFound(format!("Hospital with id {} not found", hospital_id))
            }
            _ => AppError::DatabaseError(e.to_string()),
        })?;

    Ok(hospital)
}
