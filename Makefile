create-migration:
	sqlx migrate add

run-migrations:
	sqlx migrate run


start:
	cargo run