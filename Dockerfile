FROM rust:1.83.0 as builder

WORKDIR /src/afl_server
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get upgrade -y && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/afl_server /usr/local/bin/afl_server
CMD ["afl_server"]
