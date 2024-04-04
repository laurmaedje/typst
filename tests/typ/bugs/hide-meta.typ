// Test that metadata of hidden stuff stays available.

--- issue-622-hide-meta-cite ---
#set cite(style: "chicago-notes")

A pirate. @arrgh \
#set text(2pt)
#hide[
  A @arrgh pirate.
  #bibliography("/assets/bib/works.bib")
]

--- issue-622-hide-meta-outline ---
#set text(8pt)
#outline()
#set text(2pt)
#hide(block(grid(
  [= A],
  [= B],
  block(grid(
    [= C],
    [= D],
  ))
)))
