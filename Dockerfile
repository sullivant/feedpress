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

RUN rustup target add x86_64-unknown-linux-musl && \
    apt update && \
    apt install -y musl-tools musl-dev libssl-dev && \
    update-ca-certificates

# Set the working directory in the container to /app
WORKDIR /app

# Install typst
RUN cargo install --git https://github.com/typst/typst --locked typst-cli 

# Copy the Rust project files to the working directory
RUN USER=root cargo new feedpress
COPY ./app/src ./src
COPY ./app/Cargo.toml ./

RUN cargo build --release
RUN cargo install --target x86_64-unknown-linux-musl --path .

# Build the Rust app
#RUN cargo install --path .

FROM scratch
COPY --from=builder /usr/local/cargo/bin/feedpress .
COPY --from=builder /usr/loca/cargo/bin/typst /usr/local/bin/typst
USER 1000
CMD ["./feedpress --serve"]


