--- grid-footer-rowspan ---
// General footer-only tests
#set page(height: 9em)
#table(
  columns: 2,
  [a], [],
  [b], [],
  [c], [],
  [d], [],
  [e], [],
  table.footer(
    [*Ok*], table.cell(rowspan: 2)[test],
    [*Thanks*]
  )
)

--- grid-footer-bare-1 ---
#set page(height: 5em)
#table(
  table.footer[a][b][c]
)

--- grid-footer-bare-2 ---
#table(table.footer[a][b][c])

#table(
  gutter: 3pt,
  table.footer[a][b][c]
)

--- grid-footer-stroke-edge-cases ---
// Test footer stroke priority edge case
#set page(height: 10em)
#table(
  columns: 2,
  stroke: black,
  ..(table.cell(stroke: aqua)[d],) * 8,
  table.footer(
    table.cell(rowspan: 2, colspan: 2)[a],
    [c], [d]
  )
)

--- grid-footer-hline-and-vline-1 ---
// Footer should appear at the bottom. Red line should be above the footer.
// Green line should be on the left border.
#set page(margin: 2pt)
#set text(6pt)
#table(
  columns: 2,
  inset: 1.5pt,
  table.cell(y: 0)[a],
  table.cell(x: 1, y: 1)[a],
  table.cell(y: 2)[a],
  table.footer(
    table.hline(stroke: red),
    table.vline(stroke: green),
    [b],
  ),
  table.cell(x: 1, y: 3)[c]
)

--- grid-footer-hline-and-vline-2 ---
// Table should be just one row. [c] appears at the third column.
#set page(margin: 2pt)
#set text(6pt)
#table(
  columns: 3,
  inset: 1.5pt,
  table.cell(y: 0)[a],
  table.footer(
    table.hline(stroke: red),
    table.hline(y: 1, stroke: aqua),
    table.cell(y: 0)[b],
    [c]
  )
)

--- grid-footer-below-rowspans ---
// Footer should go below the rowspans.
#set page(margin: 2pt)
#set text(6pt)
#table(
  columns: 2,
  inset: 1.5pt,
  table.cell(rowspan: 2)[a], table.cell(rowspan: 2)[b],
  table.footer()
)
