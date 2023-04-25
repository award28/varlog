FROM varlog/base:latest

RUN rustup target add wasm32-unknown-unknown

COPY ./app ./app

WORKDIR /usr/src/app

RUN trunk build --release

CMD ["trunk", "serve", "--release", "--address=0.0.0.0", "--port=8000", "--proxy-backend=http://server:8080/v1"]
