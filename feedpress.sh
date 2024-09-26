#!/bin/bash

cd /app || exit
cargo run -- --serve
#cd /
#typst compile templates/feedpress.typ output/feedpress.pdf --root ./
