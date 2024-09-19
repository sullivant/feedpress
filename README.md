# feedpress
RSS to Newspaper Tooling

## Planning and Proof of Concepts
- ~Source RSS text from list and pull out content~
- ~[typst](https://github.com/typst/typst) formatting and layout with callouts/includes to sections?~
- feedpress.dev - site thoughts - should it be a single page with links to tooling releases?  A service? 
- auto generate the proper biblios so we can see a section at the end instead of links intermixed.
- create CLI flags to run the workflow, and a flag to add a feed URL to the config file
- ~add feed configs so they can be put into sections "sports" "news" etc and show the sections in the PDF~
- other tweaks to layout, etc.
- make it runable in a docker container on a cron job
- while in container, present a config webpage to add feeds, set options, etc.

## Flow

- Source feeds
- Extract and process
- Collect and organize/layout
- Export
