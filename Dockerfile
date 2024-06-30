FROM rust:1.75 as builder
WORKDIR /usr/src/bubble_backend
COPY . .

RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && update-ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/bubble_backend /usr/local/bin/bubble_backend

EXPOSE 8060

CMD ["bubble_backend"]