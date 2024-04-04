// Test page breaks on basically empty pages.

--- pagebreak-weak-place ---
// After place
// Should result in three pages.
First
#pagebreak(weak: true)
#place(right)[placed A]
#pagebreak(weak: true)
Third

--- pagebreak-weak-meta ---
// After only ignorables & invisibles
// Should result in two pages.
First
#pagebreak(weak: true)
#counter(page).update(1)
#metadata("Some")
#pagebreak(weak: true)
Second

--- pagebreak-meta ---
// After only ignorables, but regular break
// Should result in three pages.
First
#pagebreak()
#counter(page).update(1)
#metadata("Some")
#pagebreak()
Third
