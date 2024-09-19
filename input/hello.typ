#import "../templates/feedpress.typ" : conf

#let bookshelf(articles) = {
  for article in {articles.content} [
    = #article.title  
    #line(length:100%)

    #cite(label(article.bibKey)) #article.content
  ]
}

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
  doc,
)

#bookshelf(
  toml("hello.toml")
)

#line(length:100%)
#bibliography("hello-bib.yml")



