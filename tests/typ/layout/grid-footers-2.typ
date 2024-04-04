--- grid-header-footer-block-with-fixed-height ---
#set page(height: 17em)
#table(
  rows: (auto, 2.5em, auto),
  table.header[*Hello*][*World*],
  block(width: 2em, height: 10em, fill: red),
  table.footer[*Bye*][*World*],
)

--- grid-header-footer-and-rowspan-non-contiguous-1 ---
// Rowspan sizing algorithm doesn't do the best job at non-contiguous content
// ATM.
#set page(height: 20em)

#table(
  rows: (auto, 2.5em, 2em, auto, 5em, 2em, 2.5em),
  table.header[*Hello*][*World*],
  table.cell(rowspan: 3, lorem(20)),
  table.footer[*Ok*][*Bye*],
)

--- grid-header-footer-and-rowspan-non-contiguous-2 ---
// This should look right
#set page(height: 20em)

#table(
  rows: (auto, 2.5em, 2em, auto),
  gutter: 3pt,
  table.header[*Hello*][*World*],
  table.cell(rowspan: 3, lorem(20)),
  table.footer[*Ok*][*Bye*],
)
