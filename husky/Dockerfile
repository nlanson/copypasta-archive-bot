FROM rust as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM rust as runtime
WORKDIR /app
COPY --from=builder /app/target/release/husky .
EXPOSE 8000
CMD ["/app/husky"]