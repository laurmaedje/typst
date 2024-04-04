// Test box in 100% width block.

--- issue-2128-block-width-box ---
#block(width: 100%, fill: red, box("a box"))

#block(width: 100%, fill: red, [#box("a box") #box()])
