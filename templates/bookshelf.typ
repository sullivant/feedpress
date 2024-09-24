#let alert(body, fill: red) = {
  set text(white)
  set align(center)
  rect(
    fill: fill,
    inset: 8pt,
    radius: 4pt,
    [*#body*],
  )
}

#let bookshelf(articles) = {
  // Sort the articles by their section
  let sorted = articles.content.sorted(key: k => k.at("section"))

  let current_section = ""
  // Then display the content
  for article in {sorted} [
    #if article.section != current_section {
      current_section = article.section
      [= *#article.section*]
    }
    #set align(left)
    == #article.title
    #line(length:100%)
    #cite(label(article.bib_key)) #article.content
    //#cite(label(article.bib_key)) #eval(article.content, mode: "markup")
  ]
}

