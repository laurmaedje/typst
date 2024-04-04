// Test SVG with text.

--- svg-text ---
#set page(width: 250pt)

#figure(
  image("/assets/images/diagram.svg"),
  caption: [A textful diagram],
)

--- svg-text-font ---
#set page(width: 250pt)
#show image: set text(font: ("Roboto", "Noto Serif CJK SC"))

#figure(
  image("/assets/images/chinese.svg"),
  caption: [Bilingual text]
)
