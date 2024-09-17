#import "../templates/feedpress.typ" : conf


#show: doc => conf(
  title: [Feed Press],
  dateStamp: [Today's date is #datetime.today().display()],
  version: [feedPress v0.0.1],
  authors: (
      (
        name: "Thomas Sullivan",
        affiliation: "Sullivan Scientific",
        email: "thomas@sullivanscientific.net",
      ),
  ),
  abstract: [#lorem(10)],
  doc,
)

= Introduction @harry
#lorem(300)

= Related Work @electronic
#lorem(200)

= Next Article @harry @electronic
#lorem(400)

= TODOs
- Sectional bylines with source name in short form and bib link
- Consider what to put in abstract area

#line(length:100%)
#bibliography("hello.yml")
