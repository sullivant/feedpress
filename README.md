# feedpress
RSS to Newspaper Tooling

## Flow
- Source feeds
- Extract and process
- Collect and organize/layout
- Export

## CLI Requirements
- [typst](https://github.com/typst/typst) - for formatting and layout processing.
- Rust and Cargo - for compilation and running via `cargo run`

## Planning and Proof of Concepts
- ~Source RSS text from list and pull out content~
- ~[typst](https://github.com/typst/typst) formatting and layout with callouts/includes to sections?~
- ~add feed configs so they can be put into sections "sports" "news" etc and show the sections in the PDF~
- ~allow for date filtering - max lookback, etc.~
- auto generate the proper biblios so we can see a section at the end instead of links intermixed.
- create CLI flags to run the workflow, and a flag to add a feed URL to the config file
- other tweaks to layout, etc.
- strip any inline img and other tags
- create usage case and example documentation for how to configure or adjust feed entries, etc
- *maybe*: consider allowing it to run with a file per day, so there are archived copies available, etc.  Instead of `input.toml` and `input-bib.toml` allow for it to be dated and reflected as params into `feedpress.typ` and then output to `date.pdf`?

## Containerization
- make it runable in a docker container on a cron job
- while in container, present a config webpage to add feeds, set options, etc.

## feedpress.dev
- Should it be a single readme with the usual "it does things" information?
- A hostable service some day?