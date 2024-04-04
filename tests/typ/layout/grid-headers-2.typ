--- grid-header-block-with-fixed-height ---
#set page(height: 15em)
#table(
  rows: (auto, 2.5em, auto),
  table.header(
    [*Hello*],
    [*World*]
  ),
  block(width: 2em, height: 20em, fill: red)
)

--- grid-header-and-rowspan-non-contiguous-1 ---
// Rowspan sizing algorithm doesn't do the best job at non-contiguous content
// ATM.
#set page(height: 15em)

#table(
  rows: (auto, 2.5em, 2em, auto, 5em),
  table.header(
    [*Hello*],
    [*World*]
  ),
  table.cell(rowspan: 3, lorem(40))
)

--- grid-header-and-rowspan-non-contiguous-2 ---
// Rowspan sizing algorithm doesn't do the best job at non-contiguous content
// ATM.
#set page(height: 15em)

#table(
  rows: (auto, 2.5em, 2em, auto, 5em),
  gutter: 3pt,
  table.header(
    [*Hello*],
    [*World*]
  ),
  table.cell(rowspan: 3, lorem(40))
)

--- grid-header-and-rowspan-non-contiguous-3 ---
// This should look right
#set page(height: 15em)

#table(
  rows: (auto, 2.5em, 2em, auto),
  gutter: 3pt,
  table.header(
    [*Hello*],
    [*World*]
  ),
  table.cell(rowspan: 3, lorem(40))
)
