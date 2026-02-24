use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

#[derive(Debug, Serialize, FromRow)]
pub struct Doctor {
    pub id: Uuid,
    pub name: String,
    pub specialization: String,
    pub visiting_hours: String,
    pub available_days: Vec<String>,
}

#[derive(Deserialize)]
pub struct CreateDoctor {
    pub name: String,
    pub specialization: String,
    pub visiting_hours: String,
    pub available_days: Vec<String>,
}

#[derive(Serialize)]
pub struct DoctorList {
    pub doctors: Vec<Doctor>,
}

// #[derive(Deserialize)]
// pub struct CheckAvailability {
//     pub doctor_id: Uuid,
//     pub date: String,
//     pub day: String,
// }

// #[derive(Deserialize)]
// pub struct CheckAvailable {
//     pub date: String,
//     pub day: String,
// }

impl Doctor {
    pub fn new(data: CreateDoctor) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: data.name,
            specialization: data.specialization,
            visiting_hours: data.visiting_hours,
            available_days: data.available_days,
        }
    }
}
