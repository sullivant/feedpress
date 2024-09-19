#let bookshelf(articles) = {
  for article in {articles.content} [
    = #article.title  
    #line(length:100%)

    #cite(label(article.bib_key)) #article.content
  ]
}

