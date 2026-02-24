use crate::utils::create_random_string;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, FromRow, Serialize)]
pub struct Patient {
    pub id: Uuid,
    pub name: String,
    pub age: i64,
    pub card_id: String,
    pub gender: String,
}

#[derive(Deserialize)]
pub struct CreatePatient {
    name: String,
    age: i64,
    gender: String,
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
            gender: data.gender,
        }
    }
}
