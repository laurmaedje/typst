// Test clipping with the `box` and `block` containers.

--- box-clip-rect ---
// Test box clipping with a rectangle
Hello #box(width: 1em, height: 1em, clip: false)[#rect(width: 3em, height: 3em, fill: red)]
world 1

Space

Hello #box(width: 1em, height: 1em, clip: true)[#rect(width: 3em, height: 3em, fill: red)]
world 2

--- block-clip-text ---
// Test cliping text
#block(width: 5em, height: 2em, clip: false, stroke: 1pt + black)[
  But, soft! what light through
]

#v(2em)

#block(width: 5em, height: 2em, clip: true, stroke: 1pt + black)[
  But, soft! what light through yonder window breaks? It is the east, and Juliet
  is the sun.
]

--- block-clip-svg-glyphs ---
// Test clipping svg glyphs
Emoji: #box(height: 0.5em, stroke: 1pt + black)[🐪, 🌋, 🏞]

Emoji: #box(height: 0.5em, clip: true, stroke: 1pt + black)[🐪, 🌋, 🏞]

--- block-clipping-multiple-pages ---
// Test block clipping over multiple pages.
#set page(height: 60pt)

First!

#block(height: 4em, clip: true, stroke: 1pt + black)[
  But, soft! what light through yonder window breaks? It is the east, and Juliet
  is the sun.
]

--- box-clip-radius ---
// Test clipping with `radius`.
#set page(height: 60pt)

#box(
  radius: 5pt,
  stroke: 2pt + black,
  width: 20pt,
  height: 20pt,
  clip: true,
  image("/assets/images/rhino.png", width: 30pt)
)

--- box-clip-radius-without-stroke ---
// Test clipping with `radius`, but without `stroke`.
#set page(height: 60pt)

#box(
  radius: 5pt,
  width: 20pt,
  height: 20pt,
  clip: true,
  image("/assets/images/rhino.png", width: 30pt)
)
