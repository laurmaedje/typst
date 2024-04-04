// Ensure gutter rows at the top or bottom of a region are skipped.

--- issue-grid-gutter-skip ---
#set page(height: 10em)

#table(
  row-gutter: 1.5em,
  inset: 0pt,
  rows: (1fr, auto),
  [a],
  [],
  [],
  [f],
  [e\ e],
  [],
  [a]
)
