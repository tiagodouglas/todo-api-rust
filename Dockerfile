FROM rust:latest AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/target/release/api .

EXPOSE 5500

CMD ["./api"]