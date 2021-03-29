FROM rust:1.51.0 as builder
WORKDIR /usr/src/reet
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get -y install libssl-dev ca-certificates
COPY --from=builder /usr/local/cargo/bin/reet /usr/local/bin/reet
CMD ["reet"]
