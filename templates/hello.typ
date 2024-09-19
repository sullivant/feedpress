#import "feedpress.typ" : conf

#let bookshelf(articles) = {
  for article in {articles.content} [
    = #article.title  
    #line(length:100%)

    #cite(label(article.bib_key)) #article.content
  ]
}

#show: doc => conf(
  title: [Feed Press],
  dateStamp: [Today's date is #datetime.today().display()],
  version: [feedPress v0.0.1],
  doc,
)

#bookshelf(
  toml("../input/hello.toml")
)

#line(length:100%)
#bibliography("../input/hello-bib.yml")



