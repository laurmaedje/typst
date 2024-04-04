// Test if text with region works

--- text-lang ---
// without any region
#set text(font: "Noto Serif CJK TC", lang: "zh")
#outline()

--- text-lang-unknown-region ---
// with unknown region configured
#set text(font: "Noto Serif CJK TC", lang: "zh", region: "XX")
#outline()

--- text-lang-region ---
// with region configured
#set text(font: "Noto Serif CJK TC", lang: "zh", region: "TW")
#outline()
