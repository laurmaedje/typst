// Test layout transformations

--- transform-rotate ---
// Test that rotation impact layout.
#set page(width: 200pt)
#set rotate(reflow: true)

#let one(angle) = box(fill: aqua, rotate(angle)[Test Text])
#for angle in range(0, 360, step: 15) {
  one(angle * 1deg)
}

--- transform-rotate-relative-sizing ---
// Test relative sizing in rotated boxes.
#set page(width: 200pt, height: 200pt)
#set text(size: 32pt)
#let rotated(body) = box(rotate(
  90deg,
  box(stroke: 0.5pt, height: 20%, clip: true, body)
))

#set rotate(reflow: false)
Hello #rotated[World]!\

#set rotate(reflow: true)
Hello #rotated[World]!

--- transform-scale ---
// Test that scaling impact layout.
#set page(width: 200pt)
#set text(size: 32pt)
#let scaled(body) = box(scale(
  x: 20%,
  y: 40%,
  body
))

#set scale(reflow: false)
Hello #scaled[World]!

#set scale(reflow: true)
Hello #scaled[World]!

--- transform-scale-relative-sizing ---
// Test relative sizing in scaled boxes.
#set page(width: 200pt, height: 200pt)
#set text(size: 32pt)
#let scaled(body) = box(scale(
  x: 60%,
  y: 40%,
  box(stroke: 0.5pt, width: 30%, clip: true, body)
))

#set scale(reflow: false)
Hello #scaled[World]!\

#set scale(reflow: true)
Hello #scaled[World]!
