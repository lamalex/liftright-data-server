FROM ekidd/rust-musl-builder:stable as builder

RUN USER=root cargo new --bin rust-docker-web
WORKDIR ./rust-docker-web
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./

RUN rm ./target/x86_64-unknown-linux-musl/release/deps/rust_docker_web*
RUN cargo build --release


FROM alpine:latest