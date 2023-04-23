FROM varlog/base:latest

COPY ./server ./server

RUN cargo install --path ./server

CMD ["varlog_server"]
