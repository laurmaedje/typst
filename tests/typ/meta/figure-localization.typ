// Test localization-related figure features.

--- figure-localization-fr ---
// Test French
#set text(lang: "fr")
#figure(
  circle(),
  caption: [Un cercle.],
)

--- figure-localization-zh ---
// Test Chinese
#set text(lang: "zh")
#figure(
  rect(),
  caption: [一个矩形],
)

--- figure-localization-ru ---
// Test Russian
#set text(lang: "ru")

#figure(
    polygon.regular(size: 1cm, vertices: 8),
    caption: [Пятиугольник],
)

--- figure-localization-gr ---
// Test Greek
#set text(lang: "gr")
#figure(
  circle(),
  caption: [Ένας κύκλος.],
)
