#import "layout.typ" : conf
#import "bookshelf.typ" : bookshelf

#let ver = toml("../app/Cargo.toml")

#show: doc => conf(
  title: [Feed Press],
  dateStamp: [Today's date is #datetime.today().display()],
  version: [feedPress #ver.package.version],
  doc,
)

#bookshelf(
  toml("../input/input.toml")
)

#colbreak()
#line(length:100%)
#bibliography("../input/input-bib.yml")



