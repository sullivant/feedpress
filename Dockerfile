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

WORKDIR /app
COPY ./app/src ./src
COPY ./app/Cargo.toml ./
RUN cargo build --release
RUN strip target/release/feedpress



FROM alpine:latest as release
WORKDIR /app
COPY --from=builder /app/target/release/feedpress .
COPY --from=builder /usr/local/cargo/bin/typst /usr/local/bin/typst

ENV ROCKET_ADDRESS=0.0.0.0
ENV ROCKET_PORT=8081
EXPOSE 8081

CMD ["./feedpress --serve"]


