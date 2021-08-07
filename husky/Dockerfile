FROM rust as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM rust as runtime
WORKDIR /app
COPY --from=builder /app/target/release/husky .
COPY --from=builder /app/pastas.db .
EXPOSE 8000
CMD ["./husky"]

# Issues
#  - Cannot get the program to start with applied address configuration. 
#    The program is able to use the selected log-level but is unable to 
#    apply the inputted address config. (See rocket.toml for details)