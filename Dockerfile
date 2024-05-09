FROM rust:latest as builder

WORKDIR /usr/src/fochan

COPY . .

RUN rustup default nightly && cargo build --release

FROM ubuntu:latest

RUN apt-get update && apt-get install -y libssl-dev

COPY --from=builder /usr/src/fochan/target/release/fochan /usr/local/bin/fochan

CMD ["fochan"]
