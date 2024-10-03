# <img src="https://github.com/sullivant/feedpress/blob/main/assets/logo.jpg?raw=true" height=100 width=100> feedpress
RSS to Newspaper Tooling

## The Pitch
I am a big fan of RSS feeds and readers - in fact my current setup involves using a self-hosted feed processing container [freshrss](https://freshrss.org) which is then "read" by an iOS app, [reeder](https://reederapp.com/).

However it was occurring to me that sometimes I'd like a simple, collected, PDF with articles I found most interesting, in a newspaper format.  Even if I simply read this PDF on an ipad or sent it to a printer each day, I think I'd enjoy that.

Enter **feedpress**:

- [x] Pull a few articles from curated RSS feeds
- [x] Process them, cleaning up as much crap as possible
- [x] Output a *typst* formatted file - and combined with layout/templating, use typst to create a PDF

Documentation: ./docs/index.html

## Requisites
### Building
- rust & cargo (https://rust-lang.org)
- typst (https://typst.app)
- git (https://git-scm.com)
### Development
- rust & cargo
```bash
# Build in your normal and comforable way, serve via cargo:
cargo run -- --serve
```
- typst
```bash
# Set typst to watch the input files, so you can tinker with layouts:
typst watch templates/feedpress.typ output/feedpress.pdf --root ./
```
- git
- tailwindcss (https://tailwindcss.com) (for UI elements) 
```bash
npx tailwindcss -i assets/static/input.css -o assets/static/output.css --watch
```

## Running and Configuration
Get the code: 
`git@github.com:sullivant/feedpress.git`

Update some configuration.  After checkout of this repository, note that there are a few "defaults" configured and a few example feeds.  Without parameters, the application will run and serve a simple front end, available at `http://localhost:8081/`.

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
# Will execute a feed pull and create input files suitable for typst.
cargo run --release

# Will serve a webpage located at localhost:8081/
cargo run -- --serve

# Will show options
cargo run -- --help
```

The directories in this project are as follows:
```
./ - Root directory of this repository
./app - Contains the code to feedpress and the target binary
./assets - Contains local images used for logos, served webpage, etc.
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
- [ ] Unit tests
- [ ] Documentation of all functions
- [ ] Proper error "match" handling, with Result<> etc.
- [x] Dockerization of build process
- [x] Releases and runnability on its own in a container
- [x] Output should be datestamped in PDF name, not static
- [x] On container startup or `cargo run` default behavior should be to serve the static site
- [x] But still allow for parameterized CLI execution
- [x] When in container mode, there should be a static page showing detail of configuration, a "run now" button, and later a scheduled task?
### feedpress.dev
- [ ] Create static markdown driven site - that's just this readme at first?