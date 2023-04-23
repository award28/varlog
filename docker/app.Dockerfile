FROM varlog/base:latest

COPY ./app ./app

RUN cargo install --path ./app

CMD ["varlog_app"]
