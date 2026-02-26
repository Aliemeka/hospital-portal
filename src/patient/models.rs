use crate::utils::create_random_string;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize)]
pub struct Patient {
    pub id: Uuid,
    pub name: String,
    pub age: i32,
    pub card_id: String,
    pub gender: String,
    pub hospital_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct CreatePatient {
    name: String,
    age: i32,
    gender: String,
    hospital_id: Option<Uuid>,
    user_id: Option<Uuid>,
}

#[derive(Serialize)]
pub struct PatientList {
    pub patients: Vec<Patient>,
}

impl Patient {
    pub fn new(data: CreatePatient) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: data.name,
            age: data.age,
            card_id: create_random_string(10),
            hospital_id: data.hospital_id,
            gender: data.gender,
            user_id: data.user_id,
        }
    }
}
