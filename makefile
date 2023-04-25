.PHONY: app server clean registry

book:
	cd docs && mdbook serve --open

server:
	cargo run -p server

registry:
	cargo run -p registry

app:
	trunk serve --proxy-backend=http://localhost:8080/v1 --address=0.0.0.0 --port=8000 app/index.html

clean:
	docker rmi varlog/base varlog/app varlog/server

build:
	docker build -f ./docker/base.Dockerfile . --tag varlog/base 
	docker build -f ./docker/registry.Dockerfile . --tag varlog/registry 
	docker build -f ./docker/server.Dockerfile . --tag varlog/server 
	docker build -f ./docker/app.Dockerfile . --tag varlog/app 

up:
	docker compose up -d

fresh: build up

down:
	docker compose down

ps:
	docker compose ps

logs:
	docker compose logs -f
