FROM rust:latest as builder

WORKDIR /src/afl_server
COPY . .
RUN cargo install --path .

FROM debian:bookworm-slim
RUN apt-get update && apt-get upgrade -y && apt-get install -y openssl ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/afl_server /usr/local/bin/afl_server
ENV AFL_USER_EMAIL=david.14587@gmail.com
CMD ["afl_server"]
