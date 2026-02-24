CREATE TABLE IF NOT EXISTS appointments (
    id UUID PRIMARY KEY,
    patient_id UUID NOT NULL,
    doctor_id UUID NOT NULL,
    time TIMESTAMPTZ NOT NULL,
    purpose TEXT NOT NULL,
    status VARCHAR(100) NOT NULL,
    price NUMERIC(10, 2) NOT NULL,
    FOREIGN KEY (patient_id) REFERENCES patients(id) ON DELETE CASCADE,
    FOREIGN KEY (doctor_id) REFERENCES doctors(id) ON DELETE CASCADE
);

-- pub struct Appointment {
--     pub id: Uuid,
--     pub patient_id: Uuid,
--     pub doctor_id: Uuid,
--     pub purpose: String,
--     pub time: DateTime<Utc>,
--     pub status: AppointmentStatus,
--     pub price: f64,
-- }