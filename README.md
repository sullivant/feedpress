# feedpress
RSS to Newspaper Tooling


## Configuration
- After checkout of this repository, note that there are a few "defaults" configured and a few example feeds.

- Configuration is located in `data/config.toml`
```toml
# global configurations
show_errors = false # true if you'd like to see feed collection errors
max_age = 3 # number of days old an article is skipped if it is older than
feed_limit = 2  # max number of articles to pull from a feed

# feed array
# example:
# [[feed]]
#   url = "https://yourfeedurl/rss.xml"
#   feed_limit = 10
#   section = "Personal"

```

From the app directory, you can run this program.  It will refer to the parent directory for configurations, etc.

```bash
cargo run --release
```

The directories in this project are as follows:
```
./ - Root directory of this repository
./app - Contains the code to feedpress and the target binary
./assets - Contains local images used for logos, etc
./data - Contains configuration 
./docs - Documentation
./input - Input that is feed into typst
./output - Output PDF files for viewing or delivery
./templates - The actual typst configuration
./templates/bookshelf.typ - How each article looks on the page
./templates/feedpress.typ - The root typst "application"; imports the other .typ files
./templates/layout.typ - The overall layout of the page
```

## Flow
- Source feeds
- Extract and process
- Collect and organize/layout
- Export

## CLI Requirements
- [typst](https://github.com/typst/typst) - for formatting and layout processing.
- Rust and Cargo - for compilation and running via `cargo run`


## Next Steps

### Planning and Proof of Concepts
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

### Containerization
- make it runable in a docker container on a cron job
- while in container, present a config webpage to add feeds, set options, etc.

### feedpress.dev
- Should it be a single readme with the usual "it does things" information?
- A hostable service some day?