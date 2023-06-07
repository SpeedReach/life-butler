# syntax=docker/dockerfile:1
   
FROM rust:1.69 as builder
WORKDIR /usr/src/life-butler
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/life-butler /usr/local/bin/life-butler
CMD ["life-butler"]
EXPOSE 8080