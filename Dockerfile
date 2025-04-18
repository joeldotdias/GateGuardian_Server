FROM rust:latest AS builder

RUN user=root cargo new --bin ggserver
WORKDIR /ggserver

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src
COPY ./migrations ./migrations

RUN rm ./target/release/deps/ggserver*
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /ggserver/target/release/ggserver /
COPY --from=builder /ggserver/migrations /
CMD [ "./ggserver" ]
EXPOSE 6969
