// Test alignment.

--- align-in-stack ---
#set page(height: 100pt)
#stack(dir: ltr,
  align(left, square(size: 15pt, fill: eastern)),
  align(center, square(size: 20pt, fill: eastern)),
  align(right, square(size: 15pt, fill: eastern)),
)
#align(center + horizon, rect(fill: eastern, height: 10pt))
#align(bottom, stack(
  align(center, rect(fill: conifer, height: 10pt)),
  rect(fill: forest, height: 10pt, width: 100%),
))

--- align-center-in-flow ---
// Test that multiple paragraphs in subflow also respect alignment.
#align(center)[
  Lorem Ipsum

  Dolor
]

--- align-start-and-end ---
// Test start and end alignment.
#rotate(-30deg, origin: end + horizon)[Hello]

#set text(lang: "de")
#align(start)[Start]
#align(end)[Ende]

#set text(lang: "ar")
#align(start)[يبدأ]
#align(end)[نهاية]

--- alignment-type ---
#test(type(center), alignment)
#test(type(horizon), alignment)
#test(type(center + horizon), alignment)

--- alignment-add-two-horizontal ---
// Error: 8-22 cannot add two horizontal alignments
#align(center + right, [A])

--- alignment-add-two-vertical ---
// Error: 8-20 cannot add two vertical alignments
#align(top + bottom, [A])

--- alignment-add-vertical-and-2d ---
// Error: 8-30 cannot add a vertical and a 2D alignment
#align(top + (bottom + right), [A])
