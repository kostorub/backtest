FROM rust:1.77 as builder

WORKDIR /opt/app
COPY . .

RUN cargo install --path .

FROM debian:12.5-slim

RUN apt-get update && apt install -y openssl

WORKDIR /opt/app

COPY --from=builder /usr/local/cargo/bin/backtest .
COPY src/web/ ./src/web
COPY config/ ./config

ENV RUST_LOG=info
ENV ENV=production

EXPOSE 8080

CMD ["/opt/app/backtest"]
