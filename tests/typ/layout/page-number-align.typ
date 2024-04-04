// Test page number alignment.

--- page-number-align-top-right ---
#set page(
  height: 100pt,
  margin: 30pt,
  numbering: "(1)",
  number-align: top + right,
)

#block(width: 100%, height: 100%, fill: aqua.lighten(50%))

--- page-number-align-bottom-left ---
#set page(
  height: 100pt,
  margin: 30pt,
  numbering: "[1]",
  number-align: bottom + left,
)

#block(width: 100%, height: 100%, fill: aqua.lighten(50%))

--- page-number-align-left-horizon ---
// Error: 25-39 expected `top` or `bottom`, found horizon
#set page(number-align: left + horizon)
