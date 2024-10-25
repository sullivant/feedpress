FROM rust:latest AS builder

## This is a development image meant to allow for compilation of the feedpress application
## as well as processing of the output created after an RSS extraction via the installed
## typst application.  Soup to nuts, things can be run here.
## 
## Just map your directories needed
## ./assets
## ./data
## ./input
## ./output
## ./templates

## For running just the feedpress->typst application itself, without building, that'll be another
## Dockerfile, at a later date.  

# Install typst
RUN cargo install --git https://github.com/typst/typst --locked typst-cli 

# Install musl stuff
# RUN apt update && apt install -y musl-tools libssl-dev openssl && rm -rf /var/lib/apt/lists/*
# RUN rustup target add x86_64-unknown-linux-musl

WORKDIR /app
COPY ./app/src ./src
COPY ./app/config ./config
COPY ./app/Cargo.toml ./
COPY ./app/Cargo.lock ./
RUN cargo build --release 
RUN strip target/release/feedpress






# FROM debian:bullseye-slim as release
FROM rust:latest as release

RUN useradd feedpress -u 1000

WORKDIR /app

## Setup the things we need for normal operation, docker-compose users can
## if desired, map the /output (or any other) directories.
COPY ./assets/ /assets/
COPY ./data/ /data/
COPY ./input/ /input/
COPY ./output/ /output/
COPY ./templates/ /templates/
COPY ./feedpress.sh /feedpress.sh
RUN chmod +x /feedpress.sh

COPY --from=builder /app/config/log4rs.yaml /app/config/log4rs.yaml
COPY --from=builder /app/Cargo.toml /app/Cargo.toml
COPY --from=builder /app/target/release/feedpress .
COPY --from=builder /usr/local/cargo/bin/typst /usr/local/bin/typst

RUN apt update && apt install -y poppler-utils && rm -rf /var/lib/apt/lists/*

## Runtime necessary files
ENV ARCH x86_64
# # COPY --from=builder /usr/lib/${ARCH}-linux-gnu/* /usr/lib/${ARCH}-linux-gnu/
# COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libxml2* /usr/lib/${ARCH}-linux-gnu/
# COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libgcc* /usr/lib/${ARCH}-linux-gnu/
# COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libicuuc* /usr/lib/${ARCH}-linux-gnu/
# COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libz* /usr/lib/${ARCH}-linux-gnu/
# COPY --from=builder /usr/lib/${ARCH}-linux-gnu/liblz* /usr/lib/${ARCH}-linux-gnu/
# COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libicudata* /usr/lib/${ARCH}-linux-gnu/
# COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libstdc* /usr/lib/${ARCH}-linux-gnu/
# COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libssl* /usr/lib/${ARCH}-linux-gnu/
# COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libcrypto* /usr/lib/${ARCH}-linux-gnu/
# COPY --from=builder /usr/lib/${ARCH}-linux-gnu/libc* /usr/lib/${ARCH}-linux-gnu/



ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8081
EXPOSE 8081

CMD ["/feedpress.sh"]
