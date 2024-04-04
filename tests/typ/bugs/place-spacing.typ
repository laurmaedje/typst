// Test that placed elements don't add extra block spacing.

--- issue-2199-place-spacing-bottom ---
#show figure: set block(spacing: 4em)

Paragraph before float.
#figure(rect(), placement: bottom)
Paragraph after float.

--- issue-2199-place-spacing-default ---
#show place: set block(spacing: 4em)

Paragraph before place.
#place(rect())
Paragraph after place.
