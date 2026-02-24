CREATE TABLE IF NOT EXISTS doctors (
    id UUID PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    specialization VARCHAR(100) NOT NULL,
    visiting_hours VARCHAR(50) NOT NULL,
    available_days TEXT[] NOT NULL
);
