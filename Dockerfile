FROM rust:latest as builder

WORKDIR /usr/src/fochan

COPY . .

RUN rustup default nightly

RUN cargo build

FROM ubuntu:latest

RUN apt-get update && apt-get install -y libssl-dev libpq-dev

COPY --from=builder /usr/src/fochan/target/debug/fochan /usr/local/bin/fochan

CMD ["fochan"]
