.PHONY: app server

book:
	cd docs && mdbook serve --open

server:
	cargo run -p server

server-prod:
	cargo run --release -p server

app:
	cargo run -p app

app-prod:
	cargo run --release -p app

build:
	docker compose build

up:
	docker compose up -d

fresh: build up

down:
	docker compose down

ps:
	docker compose ps
