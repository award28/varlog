FROM rust:1.67 AS builder

# note that this might take a while to install, because it compiles everything from scratch
# Trunk also provides prebuilt binaries for a number of major package managers
# See https://trunkrs.dev/#install for further details
RUN cargo install --locked trunk

WORKDIR /usr/src/

COPY Cargo.toml Cargo.lock ./

RUN mkdir registry && echo echo "fn main() {}" > ./registry/main.rs
COPY ./registry ./registry

RUN mkdir server && echo echo "fn main() {}" > ./server/main.rs
COPY ./server ./server

RUN mkdir app && echo echo "fn main() {}" > ./app/main.rs
COPY ./app ./app

