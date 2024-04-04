// Ensure that a footnote and the first line of its entry
// always end up on the same page.

--- footnote-invariant ---
#set page(height: 120pt)

#lorem(13)

There #footnote(lorem(20))
