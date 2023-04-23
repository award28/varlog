book:
	cd docs && mdbook serve --open

run:
	cargo run

run-prod:
	cargo run --release

build:
	docker compose build

up:
	docker compose up -d

fresh: build up

down:
	docker compose down

ps:
	docker compose ps
