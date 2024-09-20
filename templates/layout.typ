#let conf(
  title: none,
  dateStamp: none,
  version: none,
  abstract: [],
  doc,
) = {
  set page(
    paper: "us-letter",
    footer: context [
      #set text(8pt)
      #set align(right)
      #grid(columns: (2fr, 8fr), rows: 1,
        rect(stroke: 0pt, align(left, [$dateStamp$ $version$])),
        rect(stroke: 0pt, align(horizon + center, [#counter(page).display("1 of 1", both: true, )]))
      )
    ],
  )
  set par(justify: true)
  set text(
    font: "Linux Libertine",
    size: 11pt,
  )

  
  align(center, text(17pt)[
    #grid(columns: 2, rows: 1, 
      rect(height: 7em, stroke: 0pt, align(horizon, image("../assets/logo.jpg", width: 5em, height: 5em))),
      rect(height: 7em, stroke: 0pt, align(horizon + center, [#sym.dots.h.c  *#title* #sym.dots.h.c]))
    )
    #line(length:100%)
  ])
  

  align(center)[
    #set par(justify: false)
    #align(left)[
      #outline(indent: 1em,)
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

}
