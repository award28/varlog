FROM varlog/base:latest

COPY ./registry ./registry

RUN cargo install --path ./registry

CMD ["varlog_registry"]
