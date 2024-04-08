FROM rust:1.77 as builder

WORKDIR /opt/app
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

COPY --from=builder /opt/app/target/release/backtest .
COPY src/web/ ./src/web
COPY .env ./.env

ENV RUST_LOG=info
ENV ENV=production

EXPOSE 8080

CMD ["/opt/app/backtest"]
