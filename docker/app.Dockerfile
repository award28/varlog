FROM varlog/base:latest

RUN rustup target add wasm32-unknown-unknown

# note that this might take a while to install, because it compiles everything from scratch
# Trunk also provides prebuilt binaries for a number of major package managers
# See https://trunkrs.dev/#install for further details
RUN cargo install --locked trunk

COPY ./app ./app

WORKDIR /usr/src/app

RUN trunk build --release

CMD ["trunk", "serve", "--release", "--address=0.0.0.0", "--port=8000", "--proxy-backend=https://yew.rs/tutorial"]
