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

# Build for the server
RUN cargo build --release

# note that this might take a while to install, because it compiles everything from scratch
# Trunk also provides prebuilt binaries for a number of major package managers
# See https://trunkrs.dev/#install for further details
RUN cargo install --locked trunk


