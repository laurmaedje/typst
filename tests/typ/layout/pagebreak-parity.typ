// Test clearing to even or odd pages.

--- pagebreak-to ---
#set page(width: 80pt, height: 30pt)
First
#pagebreak(to: "odd")
Third
#pagebreak(to: "even")
Fourth
#pagebreak(to: "even")
Sixth
#pagebreak()
Seventh
#pagebreak(to: "odd")
#page[Ninth]

--- pagebreak-to-auto-sized ---
#set page(width: auto, height: auto)

// Test with auto-sized page.
First
#pagebreak(to: "odd")
Third

--- pagebreak-to-multiple-pages ---
#set page(height: 30pt, width: 80pt)

// Test when content extends to more than one page
First

Second

#pagebreak(to: "odd")

Third
