FROM rust:latest as builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY build.rs ./
COPY src/ src/
COPY lib/ lib/
COPY utils/ utils/
COPY .husky/ .husky/

RUN cargo build --release

FROM debian:bullseye-slim

COPY --from=builder /app/target/release/your_binary /usr/local/bin/your_binary

ENTRYPOINT ["/usr/local/bin/your_binary"]
CMD ["--help"]