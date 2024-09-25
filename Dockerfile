FROM rust:latest

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

# Set the working directory in the container to /my
WORKDIR /app

# Install typst
RUN cargo install --git https://github.com/typst/typst --locked typst-cli

# Copy the Rust project files to the working directory
COPY ./app .

# Our utility to run the extract and post process
COPY --chmod=0755 ./feedpress.sh ./feedpress.sh

# Build the Rust app
RUN cargo build

# Set the command to run the Rust app
CMD ./feedpress.sh
