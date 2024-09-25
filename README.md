# <img src="https://github.com/sullivant/feedpress/blob/main/assets/logo.jpg?raw=true" height=100 width=100> feedpress
RSS to Newspaper Tooling


## The Pitch
I am a big fan of RSS feeds and readers - in fact my current setup involves using a self-hosted feed processing container [freshrss](https://freshrss.org) which is then "read" by an iOS app, [reeder](https://reederapp.com/).

However it was occurring to me that sometimes I'd like a simple, collected, PDF with articles I found most interesting, in a newspaper format.  Even if I simply read this PDF on an ipad or sent it to a printer each day, I think I'd enjoy that.

Enter **feedpress**:

- [x] Pull a few articles from curated RSS feeds
- [x] Process them, cleaning up as much crap as possible
- [x] Output a *typst* formatted file - and combined with layout/templating, use typst to create a PDF

## Requisites
- rust & cargo (https://rust-lang.org)
- typst (https://typst.app)
- git (https://git-scm.com)

## Running and Configuration
Get the code: 
`git@github.com:sullivant/feedpress.git`

Update some configuration.  After checkout of this repository, note that there are a few "defaults" configured and a few example feeds.

Configuration is located in `data/config.toml`
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
./site - Contains the source for feedpress.dev
./templates - The actual typst configuration
./templates/bookshelf.typ - How each article looks on the page
./templates/feedpress.typ - The root typst "application"; imports the other .typ files
./templates/layout.typ - The overall layout of the page
```

## Next Steps
### General
- [x] Dockerization of build process
- [ ] Releases and runnability on its own in a container
- [ ] Integration of typst as a library not a separate callout
### Configuration
- [ ] CLI utility or flags to add/remove feed URLs and options.
- [ ] Container execution should involve a small served web page allowing for feed and option manipulation
### feedpress.dev
- [ ] Create static markdown driven site - that's just this readme at first?