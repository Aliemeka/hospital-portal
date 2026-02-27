use crate::utils::hash_password;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;

#[derive(Clone, Serialize, FromRow)]
pub struct Hospital {
    pub id: Uuid,
    pub name: String,
    pub address: String,
    pub phone: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Type)]
#[sqlx(type_name = "VARCHAR", rename_all = "PascalCase")]
pub enum UserRole {
    Admin,
    Doctor,
    Patient,
}

#[derive(Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: String,
    pub role: UserRole,
    pub hospital_id: Uuid,
    pub created_at: DateTime<Utc>,
}

pub struct HospitalData {
    pub hospital: Hospital,
    pub admin: User,
}

#[derive(Deserialize)]
pub struct CreateHospital {
    pub name: String,
    pub address: String,
    pub phone: String,
    pub admin_name: String,
    pub admin_email: String,
    pub admin_password: String,
}

#[derive(Deserialize)]
pub struct UpdateHospital {
    pub name: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
}

#[derive(Serialize)]
pub struct HospitalWithAdminEmail {
    pub hospital: Hospital,
    pub admin_email: String,
}

impl HospitalData {
    pub fn new(data: CreateHospital) -> Self {
        let password_hash =
            hash_password(&data.admin_password).expect("Failed to hash password for admin user");
        let hospital = Hospital {
            id: Uuid::new_v4(),
            name: data.name,
            address: data.address,
            phone: data.phone,
            created_at: Utc::now(),
        };

        let admin = User {
            id: Uuid::new_v4(),
            name: data.admin_name,
            email: data.admin_email,
            password_hash,
            role: UserRole::Admin,
            hospital_id: hospital.id,
            created_at: Utc::now(),
        };

        Self { hospital, admin }
    }
}
