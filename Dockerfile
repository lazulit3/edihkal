FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json --bin edihkal

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json --bin edihkal
COPY . .
ENV SQLX_OFFLINE true
RUN cargo build --release --bin edihkal

FROM debian:bullseye-slim AS runtime
WORKDIR /app
COPY --from=builder /app/target/release/edihkal edihkal
COPY edihkal/configuration configuration
ENV EDIHKAL_ENVIRONMENT production
ENTRYPOINT ["./edihkal"]
