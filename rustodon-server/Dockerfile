# ---- Build Stage ----
FROM rust:1.77 as builder

WORKDIR /app
COPY . .

RUN cargo build --release -p rustodon-server

# ---- Runtime Stage ----
FROM debian:bullseye-slim

WORKDIR /app

RUN apt-get update && apt-get install -y libpq5 ca-certificates && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/rustodon-server /usr/local/bin/rustodon-server

EXPOSE 3000

CMD ["rustodon-server"]
