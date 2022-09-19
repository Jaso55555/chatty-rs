FROM rust:1.63.0-slim-buster as build

RUN user=root cargo new --bin server
WORKDIR /server

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

COPY ./src ./src
RUN cargo build --release
RUN rm src/*.rs

RUN rm ./target/release/deps/server*
RUN cargo build --release

FROM debian:buster-slim

COPY --from=build /server/target/release/server .
RUN mkdir -p /config
VOLUME ["/config"]
VOLUME ["/logs"]

CMD ["/server"]



