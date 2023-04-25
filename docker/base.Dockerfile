FROM rust:1.67 AS builder

COPY ./fake_data/myspace.1gb.log /var/log/

WORKDIR /usr/src/

COPY Cargo.toml Cargo.lock ./

RUN mkdir registry && echo echo "fn main() {}" > ./registry/main.rs
COPY ./registry ./registry

RUN mkdir server && echo echo "fn main() {}" > ./server/main.rs
COPY ./server ./server

RUN mkdir app && echo echo "fn main() {}" > ./app/main.rs
COPY ./app ./app
