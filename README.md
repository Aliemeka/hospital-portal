# Hospital Portal API

A RESTful API for managing hospital operations — patients, doctors, appointments, and billing — built with Rust using Axum, SQLx, and PostgreSQL.

## Features

- **Patients** — Register and retrieve patient records
- **Doctors** — Manage doctors and their availability schedules
- **Appointments** — Book appointments with auto-assignment of available doctors, and update appointment status
- **Billing** — Issue bills for appointments and process payments via Paystack

## Tech Stack

- [Rust](https://www.rust-lang.org/)
- [Axum](https://github.com/tokio-rs/axum) — Web framework
- [SQLx](https://github.com/launchbadge/sqlx) — Async PostgreSQL driver with migrations
- [Tokio](https://tokio.rs/) — Async runtime
- [Paystack](https://paystack.com/) — Payment processing

## Getting Started

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- PostgreSQL running locally or remotely
- A [Paystack](https://paystack.com/) account and secret key

### Setup

1. **Clone the repository**

   ```bash
   git clone https://github.com/Aliemeka/hospital-portal.git
   cd hospital-portal
   ```

2. **Create a `.env` file** in the project root:

   ```env
   DATABASE_URL=your-postgres-uri
   SERVER_PORT=
   PAYSTACK_PAYMENT_URL=https://api.paystack.co
   PAYSTACK_SECRET_KEY=your_paystack_secret_key
   ```

3. **Run database migrations**

   ```bash
   cargo install sqlx-cli
   sqlx migrate run
   ```

4. **Start the server**

   ```bash
   cargo run
   ```

   The API will be available at `http://127.0.0.1:<port-in-env>`.

### API Endpoints

| Method | Path                | Description                                             |
| ------ | ------------------- | ------------------------------------------------------- |
| GET    | `/health`           | Health check                                            |
| GET    | `/patients`         | List all patients                                       |
| POST   | `/patients`         | Create a patient                                        |
| GET    | `/doctors`          | List all doctors                                        |
| POST   | `/doctors`          | Create a doctor                                         |
| GET    | `/appointments`     | List appointments (filter by `patient_id`, `doctor_id`) |
| POST   | `/appointments`     | Book an appointment                                     |
| GET    | `/appointments/:id` | Get appointment by ID                                   |
| PATCH  | `/appointments/:id` | Update appointment status                               |
| POST   | `/billing`          | Issue a bill                                            |
| POST   | `/billing/pay`      | Process payment via Paystack                            |

## Collaborators

- **Emeka** — [@Aliemeka](https://github.com/Aliemeka)
- **Salama** — [@sallyjayz](https://github.com/sallyjayz)
- **Victory** — [@pv-dsgn](https://github.com/pv-dsgn)
