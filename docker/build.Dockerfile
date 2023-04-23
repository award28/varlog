FROM rust:1.67 AS builder

WORKDIR /usr/src/

COPY Cargo.toml Cargo.lock ./

# Prep server workspace for cached install
RUN mkdir server
COPY server/Cargo.toml ./server/
RUN echo "fn main() {}" > ./server/dummy.rs
RUN sed -i 's#src/main.rs#dummy.rs#' ./server/Cargo.toml

# Prep app workspace for cached install
RUN mkdir app
COPY app/Cargo.toml ./app/
RUN echo "fn main() {}" > ./app/dummy.rs
RUN sed -i 's#src/main.rs#dummy.rs#' ./app/Cargo.toml

RUN cargo build --release
