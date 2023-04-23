FROM rust:1.67

WORKDIR /usr/src/varlog

COPY Cargo.toml .
COPY Cargo.lock .

RUN echo "fn main() {}" > dummy.rs
RUN sed -i 's#src/main.rs#dummy.rs#' Cargo.toml

RUN cargo build --release
RUN sed -i 's#dummy.rs#src/main.rs#' Cargo.toml

COPY src src

RUN cargo build --release

EXPOSE 8080

CMD ["cargo", "run", "--release"]
