# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------
FROM rust:latest as cargo-build

RUN USER=root cargo new --bin --name liftright-data-server /usr/src/lrds
WORKDIR /usr/src/lrds

COPY Cargo.lock .
COPY Cargo.toml .
RUN sed -i '/lrds_derive/d' Cargo.toml
RUN mkdir .cargo
RUN cargo vendor > .cargo/config

COPY ./src src
RUN cargo build --release
RUN cargo install --path . --verbose

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------
FROM debian:buster-slim

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata \
    && rm -rf /var/lib/apt/lists/*

ENV TZ=Etc/UTC \
    APP_USER=lrds

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER

COPY --from=cargo-build /usr/local/cargo/bin/liftright-data-server /bin
RUN chown -R $APP_USER:$APP_USER /bin

USER $APP_USER

EXPOSE 3030
CMD ["liftright-data-server"]
