// Test the `square` function.

--- square ---
// Default square.
#box(square())
#box(square[hey!])

--- square-auto-sized ---
// Test auto-sized square.
#square(fill: eastern)[
  #set text(fill: white, weight: "bold")
  Typst
]

--- square-relatively-sized-child ---
// Test relative-sized child.
#square(fill: eastern)[
  #rect(width: 10pt, height: 5pt, fill: conifer)
  #rect(width: 40%, height: 5pt, stroke: conifer)
]

--- square-contents-overflow ---
// Test text overflowing height.
#set page(width: 75pt, height: 100pt)
#square(fill: conifer)[
  But, soft! what light through yonder window breaks?
]

--- square-height-limited ---
// Test that square does not overflow page.
#set page(width: 100pt, height: 75pt)
#square(fill: conifer)[
  But, soft! what light through yonder window breaks?
]

--- square-size-width-and-height ---
// Size wins over width and height.
// Error: 09-20 unexpected argument: width
#square(width: 10cm, height: 20cm, size: 1cm, fill: rgb("eb5278"))
