FROM rust:1.77 as builder

WORKDIR /opt/app
RUN cargo install --root /opt/app/.cargo sqlx-cli --no-default-features --features sqlite
COPY Cargo.toml Cargo.lock ./
RUN mkdir ./src && echo 'fn main() { println!("Dummy!"); }' > ./src/main.rs
RUN cargo build --release
RUN rm -rf ./src
COPY . .
RUN touch -a -m ./src/main.rs
RUN cargo build --release

FROM debian:12.5-slim

RUN apt-get update && apt install -y openssl ca-certificates && update-ca-certificates

WORKDIR /opt/app

COPY --from=builder /opt/app/target/release/backtest /opt/app/.cargo/bin/sqlx ./
COPY src/web/ ./src/web
COPY .env ./.env
COPY migrations/ ./migrations
COPY scripts/sqlx-init.sh .
RUN chmod +x ./sqlx-init.sh

ENV RUST_LOG=info

EXPOSE 8080

CMD ["/opt/app/backtest"]
