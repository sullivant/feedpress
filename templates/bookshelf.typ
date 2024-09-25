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
    #line(length:100%, stroke: 1pt)

    // Trim and replacements because it's not likeing an eval as markdown..
    #let thisContent = article.content.replace("#","").trim()

    #cite(label(article.bib_key)) #thisContent #sym.qed
    #set align(center) 
    #line(length:50%, stroke: (thickness: 1pt, dash: "loosely-dashed"))
  ]
}

