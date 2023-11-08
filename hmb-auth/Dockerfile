FROM rust:1.69 as builder
WORKDIR /usr/app
COPY . .
RUN cargo install --path .

FROM alpine:3.18.4
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/myapp
CMD ["myapp"]
