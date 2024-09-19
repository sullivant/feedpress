#let conf(
  title: none,
  dateStamp: none,
  version: none,
  abstract: [],
  doc,
) = {
  set page(
    paper: "us-letter",
    header: align(
      right + horizon,
      dateStamp,
    ),
  footer: context [
    #set text(8pt)
    #set align(right)
    #counter(page).display(
      "1 of 1",
      both: true,
    )
    ],
  )
  set par(justify: true)
  set text(
    font: "Linux Libertine",
    size: 11pt,
  )
  
  align(center, text(17pt)[
    #sym.dots.h.c *#title* #sym.dots.h.c
    #line(length:100%)
  ])
  

  align(center)[
    #set par(justify: false)
    #align(left)[
      #outline()
    ]
    #line(length:100%)
  ]

  show heading: it => [
    #set align(center)
    #set text(12pt, weight: "regular")
    #block(smallcaps(it.body))
  ]
  
  set align(left)
  columns(2,doc)

  line(length:100%)

  grid(
    columns: (1fr,) * 1,
    row-gutter: 24pt,
    align: left,
    version
  )
}
