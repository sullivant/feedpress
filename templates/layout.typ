#let conf(
  title: none,
  dateStamp: none,
  version: none,
  abstract: [],
  doc,
) = {
  set page(
    paper: "us-letter",
    margin: (top: 4pt),
    footer: context [
      #set text(8pt)
      #set align(right)
      #grid(columns: (2fr, 10fr), rows: 1,
        rect(stroke: 0pt, align(horizon + left, [$version$])),
        rect(stroke: 0pt, align(horizon + center, [#counter(page).display("1 of 1", both: true, )]))
      )
    ],
  )
  set par(justify: true)
  set text(
    font: "Linux Libertine",
    size: 11pt,
  )

  // Feedpress logo
  align(center, text(17pt)[
    #grid(columns: (2fr, 10fr, 2fr), rows: (4em, 1em),
      grid.hline(),
      rect(height: 4em, stroke: 0pt, align(horizon + left, image("../assets/logo.jpg", width: 3em, height: 3em))),
      rect(height: 4em, stroke: 0pt, align(horizon + center, [#sym.dots.h.c  *#title* #sym.dots.h.c])),
      rect(height: 4em, stroke: 0pt, align(horizon + right, image("../assets/logo.jpg", width: 3em, height: 3em))),
      grid.hline(),
      grid.cell(colspan: 3, align(horizon + left, text(8pt)[$dateStamp$])),
      grid.hline(),
    )
  ])
  
  align(center)[
    #set par(justify: false)
    #align(left)[
      #outline(indent: 1em,)
    ]
    #line(length:100%)
  ]

  set page(
    paper: "us-letter",
    margin: (top: auto),
  )
  show heading: it => [
    #set align(center)
    #set text(12pt, weight: "regular")
    #block(smallcaps(it.body))
  ]
  
  set align(left)
  columns(2,doc)

}
