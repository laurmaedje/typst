// Test constructors.

--- construct-list ---
// Ensure that constructor styles aren't passed down the tree.
// The inner list should have no extra indent.
#set par(leading: 2pt)
#list(body-indent: 20pt, [First], list[A][B])

--- construct-vs-set-rule-1 ---
// Ensure that constructor styles win, but not over outer styles.
// The outer paragraph should be right-aligned,
// but the B should be center-aligned.
#set list(marker: [>])
#list(marker: [--])[
  #rect(width: 2cm, fill: conifer, inset: 4pt, list[A])
]

--- construct-vs-set-rule-2 ---
// The inner rectangle should also be yellow here.
// (and therefore invisible)
#[#set rect(fill: yellow);#text(1em, rect(inset: 5pt, rect()))]

--- construct-vs-set-rule-3 ---
// The inner rectangle should not be yellow here.
A #box(rect(fill: yellow, inset: 5pt, rect())) B

--- construct-vs-show-set-rule ---
// The constructor property should still work
// when there are recursive show rules.
#show enum: set text(blue)
#enum(numbering: "(a)", [A], enum[B])
