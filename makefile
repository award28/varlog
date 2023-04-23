.PHONY: app server clean

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

clean:
	docker rmi varlog/base varlog/app varlog/server

build:
	docker build -f ./docker/build.Dockerfile . --tag varlog/base 
	docker build -f ./docker/server.Dockerfile . --tag varlog/server 
	docker build -f ./docker/app.Dockerfile . --tag varlog/app 

up:
	docker compose up -d || (make build && make up)

fresh: build up

down:
	docker compose down

ps:
	docker compose ps

logs:
	docker compose logs -f
