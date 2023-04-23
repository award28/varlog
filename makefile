book:
	cd docs && mdbook serve --open

run:
	cargo run

run-prod:
	cargo run --release

up:
	docker compose build
	docker compose up -d

down:
	docker compose down

ps:
	docker compose ps
