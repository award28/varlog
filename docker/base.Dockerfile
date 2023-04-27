FROM rust:1.67 AS builder

WORKDIR /usr/src/

COPY Cargo.toml Cargo.lock ./

RUN mkdir registry && echo echo "fn main() {}" > ./registry/main.rs
COPY ./registry ./registry

RUN mkdir server && echo echo "fn main() {}" > ./server/main.rs
COPY ./server ./server

RUN mkdir app && echo echo "fn main() {}" > ./app/main.rs
COPY ./app ./app
