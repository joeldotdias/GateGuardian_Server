FROM rust:1.77 AS builder

RUN user=root cargo new --bin ggserver
WORKDIR /ggserver

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/deps/ggserver*
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12
COPY --from=builder /ggserver/target/release/ggserver /
CMD [ "./ggserver" ]
EXPOSE 6969