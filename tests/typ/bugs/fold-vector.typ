// Test fold order of vectors.

--- issue-vec-fold-order-text-features ---
#set text(features: (liga: 1))
#set text(features: (liga: 0))
fi

--- issue-vec-fold-order-text-decos ---
#underline(stroke: aqua + 4pt)[
  #underline[Hello]
]

--- issue-vec-fold-order-meta ---
#let c = counter("mycounter")
#c.update(1)
#locate(loc => [
  #c.update(2)
  #c.at(loc) \
  Second: #locate(loc => c.at(loc))
])
