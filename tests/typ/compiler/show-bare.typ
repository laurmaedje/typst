// Test bare show without selector.

--- show-bare-basic ---
#set page(height: 130pt)
#set text(0.7em)

#align(center)[
  #text(1.3em)[*Essay on typography*] \
  T. Ypst
]

#show: columns.with(2)
Great typography is at the essence of great storytelling. It is the medium that
transports meaning from parchment to reader, the wave that sparks a flame
in booklovers and the great fulfiller of human need.

--- show-bare-content-block ---
// Test bare show in content block.
A #[_B #show: c => [*#c*]; C_] D

--- show-bare-vs-set-text ---
// Test style precedence.
#set text(fill: eastern, size: 1.5em)
#show: text.with(fill: forest)
Forest

--- show-bare-replace-with-content ---
#show: [Shown]
Ignored

--- show-bare-in-expression ---
// Error: 4-19 show is only allowed directly in code and content blocks
#((show: body => 2) * body)

--- show-bare-missing-colon-closure ---
// Error: 6 expected colon
#show it => {}

--- show-bare-missing-colon ---
// Error: 6 expected colon
#show it
