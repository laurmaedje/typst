// Issue #2821: Setting a figure's supplement to none removes the field

--- issue-2821-missing-fields ---
#show figure.caption: it => {
  assert(it.has("supplement"))
  assert(it.supplement == none)
}
#figure([], caption: [], supplement: none)
