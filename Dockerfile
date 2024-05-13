FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef

WORKDIR /app

FROM chef AS planner

COPY . .

RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json

RUN rustup default nightly

RUN cargo +nightly chef cook --release --recipe-path recipe.json

COPY . .

RUN cargo build --release --bin fochan

FROM debian:bookworm-slim AS runtime

WORKDIR /app

COPY --from=builder /app/target/release/fochan /usr/local/bin/fochan

RUN apt-get update && apt-get install -y libssl-dev libpq-dev

CMD ["fochan"]
