CREATE TABLE IF NOT EXISTS bills (
    id UUID PRIMARY KEY,
    reference VARCHAR(100) UNIQUE NOT NULL,
    appointment_id UUID NOT NULL,
    amount NUMERIC(10, 2) NOT NULL,
    currency VARCHAR(10) NOT NULL,
    status VARCHAR(100) NOT NULL,
    FOREIGN KEY (appointment_id) REFERENCES appointments(id) ON DELETE CASCADE
);