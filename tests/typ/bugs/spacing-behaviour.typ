// Test that metadata after spacing does not force a new paragraph.

--- issue-3624-spacing-behaviour ---
#{
  h(1em)
  counter(heading).update(4)
  [Hello ]
  counter(heading).display()
}
