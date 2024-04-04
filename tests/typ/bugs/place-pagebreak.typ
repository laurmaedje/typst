// Test placing on an already full page.
// It shouldn't result in a page break.

--- issue-1368-place-pagebreak ---
#set page(height: 40pt)
#block(height: 100%)
#place(bottom + right)[Hello world]
