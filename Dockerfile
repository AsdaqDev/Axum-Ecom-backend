FROM rust:1.75 as builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:buster-slim
COPY --from=builder /usr/src/app/target/release/rust_ecom /usr/local/bin/rust_ecom
EXPOSE 8080
EXPOSE 9000  # Prometheus metrics
CMD ["rust_ecom"]
