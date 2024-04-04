// Rowspan split tests

--- grid-rowspan-split-1 ---
#set page(height: 10em)
#table(
  columns: 2,
  rows: (auto, auto, 3em),
  fill: red,
  [a], table.cell(rowspan: 3, block(width: 50%, height: 10em, fill: orange) + place(bottom)[*ZD*]),
  [e],
  [f]
)

--- grid-rowspan-split-2 ---
#set page(height: 10em)
#table(
  columns: 2,
  rows: (auto, auto, 3em),
  row-gutter: 1em,
  fill: red,
  [a], table.cell(rowspan: 3, block(width: 50%, height: 10em, fill: orange) + place(bottom)[*ZD*]),
  [e],
  [f]
)

--- grid-rowspan-split-3 ---
#set page(height: 5em)
#table(
  columns: 2,
  fill: red,
  inset: 0pt,
  table.cell(fill: orange, rowspan: 10, place(bottom)[*Z*] + [x\ ] * 10 + place(bottom)[*ZZ*]),
  ..([y],) * 10,
  [a], [b],
)

--- grid-rowspan-split-4 ---
#set page(height: 5em)
#table(
  columns: 2,
  fill: red,
  inset: 0pt,
  gutter: 2pt,
  table.cell(fill: orange, rowspan: 10, place(bottom)[*Z*] + [x\ ] * 10 + place(bottom)[*ZZ*]),
  ..([y],) * 10,
  [a], [b],
)

--- grid-rowspan-split-5 ---
#set page(height: 5em)
#table(
  columns: 2,
  fill: red,
  inset: 0pt,
  table.cell(fill: orange, rowspan: 10, breakable: false, place(bottom)[*Z*] + [x\ ] * 10 + place(bottom)[*ZZ*]),
  ..([y],) * 10,
  [a], [b],
)

--- grid-rowspan-split-6 ---
#set page(height: 5em)
#table(
  columns: 2,
  fill: red,
  inset: 0pt,
  gutter: 2pt,
  table.cell(fill: orange, rowspan: 10, breakable: false, place(bottom)[*Z*] + [x\ ] * 10 + place(bottom)[*ZZ*]),
  ..([y],) * 10,
  [a], [b],
)

--- grid-rowspan-split-7 ---
#set page(height: 5em)
#grid(
  columns: 2,
  stroke: red,
  inset: 5pt,
  grid.cell(rowspan: 5)[a\ b\ c\ d\ e]
)

--- grid-rowspan-split-8 ---
#set page(height: 5em)
#table(
  columns: 2,
  gutter: 3pt,
  stroke: red,
  inset: 5pt,
  table.cell(rowspan: 5)[a\ b\ c\ d\ e]
)
