FROM rust:1.83-alpine AS builder
RUN apk add --no-cache musl-dev
WORKDIR /src
COPY Cargo.toml Cargo.lock ./
COPY build.rs ./
COPY locale/ locale/
COPY src/ src/
RUN cargo build --release

FROM alpine:3.21
RUN apk add --no-cache git
COPY --from=builder /src/target/release/shuu /usr/local/bin/shuu
RUN git config --global --add safe.directory '*'
ENTRYPOINT ["shuu"]
