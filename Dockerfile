# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------

FROM rust:latest as cargo-build
RUN USER=root cargo new --bin liftright-data-server
WORKDIR ./liftright-data-server
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

ADD . ./
RUN rm ./target/release/deps/liftright_data_server*
RUN cargo build --release

# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------
FROM debian:buster-slim
ARG APP=/usr/src/liftright-data-server

RUN apt-get update \
    && apt-get install -y ca-certificates tzdata libpq-dev\
    && rm -rf /var/lib/apt/lists/*

EXPOSE 3030

ENV TZ=Etc/UTC \
    APP_USER=lrds

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=cargo-build /liftright-data-server/target/release/liftright-data-server ${APP}/liftright-data-server
RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./liftright-data-server"]