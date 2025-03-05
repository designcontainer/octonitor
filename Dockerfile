FROM rust:1.85.0-bookworm as builder

WORKDIR /usr/src/codeshit
COPY ./src  ./src
COPY ./Cargo.toml .
RUN mkdir ./output
RUN apt update && apt install -y libssl-dev
RUN cargo build -r --target-dir ./output

FROM debian:bookworm
RUN apt update && apt install -y libssl-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/codeshit/output/release/octonitor /bin/octonitor
COPY ./.secrets /usr/
RUN . /usr/.secrets && dd if=/dev/zero of=/usr/.secrets bs=1k count=1 && rm /usr/.secrets

CMD "octonitor"
