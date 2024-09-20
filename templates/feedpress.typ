#import "layout.typ" : conf
#import "bookshelf.typ" : bookshelf

#show: doc => conf(
  title: [Feed Press],
  dateStamp: [Today's date is #datetime.today().display()],
  version: [feedPress v0.0.1],
  doc,
)

#bookshelf(
  toml("../input/input.toml")
)

#line(length:100%)
#bibliography("../input/input-bib.yml")



