FROM varlog/base:latest

COPY ./server ./server

RUN cargo install --path ./server

CMD ["cargo", "run", "--release", "-p", "server"]
