// Test configuring paragraph properties.

--- align-right ---
// Test ragged-left.
#set align(right)
To the right! Where the sunlight peeks behind the mountain.

--- par-leading-and-block-spacing ---
// Test changing leading and spacing.
#set block(spacing: 1em)
#set par(leading: 2pt)
But, soft! what light through yonder window breaks?

It is the east, and Juliet is the sun.

--- block-spacing-table ---
// Test that paragraph spacing loses against block spacing.
// TODO
#set block(spacing: 100pt)
#show table: set block(above: 5pt, below: 5pt)
Hello
#table(columns: 4, fill: (x, y) => if calc.odd(x + y) { silver })[A][B][C][D]

--- block-spacing-maximum ---
// While we're at it, test the larger block spacing wins.
#set block(spacing: 0pt)
#show raw: set block(spacing: 15pt)
#show list: set block(spacing: 2.5pt)

```rust
fn main() {}
```

- List

Paragraph
