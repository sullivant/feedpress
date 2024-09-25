#!/bin/bash

cd /app || exit
cargo run
cd /
typst compile templates/feedpress.typ output/feedpress.pdf --root ./
