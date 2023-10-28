FROM rust:1.67 as builder
WORKDIR /usr/src/botdehoras
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/botdehoras /usr/local/bin/botdehoras
COPY --from=builder /usr/src/botdehoras/.token /usr/local/bin/.token
WORKDIR /usr/local/bin
CMD ["botdehoras"]