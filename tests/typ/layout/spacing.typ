// Test the `h` and `v` functions.

--- spacing-h-and-v ---
// Linebreak and leading-sized weak spacing are equivalent.
#box[A \ B] #box[A #v(0.65em, weak: true) B]

// Eating up soft spacing.
Inv#h(0pt)isible

// Multiple spacings in a row.
Add #h(10pt) #h(10pt) up

// Relative to area.
#let x = 25% - 4pt
|#h(x)|#h(x)|#h(x)|#h(x)|

// Fractional.
| #h(1fr) | #h(2fr) | #h(1fr) |

--- space-collapsing-with-h ---
// Test spacing collapsing before spacing.
#set align(right)
A #h(0pt) B #h(0pt) \
A B \
A #h(-1fr) B

--- block-spacing-collapse-text-style ---
// Test spacing collapsing with different font sizes.
#grid(columns: 2)[
  #text(size: 12pt, block(below: 1em)[A])
  #text(size: 8pt, block(above: 1em)[B])
][
  #text(size: 12pt, block(below: 1em)[A])
  #text(size: 8pt, block(above: 1.25em)[B])
]

--- spacing-rtl ---
// Test RTL spacing.
#set text(dir: rtl)
A #h(10pt) B \
A #h(1fr) B

--- spacing-missing-amount ---
// Missing spacing.
// Error: 10-13 missing argument: amount
Totally #h() ignored
