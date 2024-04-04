// Test embedded expressions.

--- let-binding-keyword-in-markup ---
// Error: 6-8 expected pattern, found keyword `as`
// Hint: 6-8 keyword `as` is not allowed as an identifier; try `as_` instead
#let as = 1 + 2

--- let-binding-keyword-in-code ---
#{
  // Error: 7-9 expected pattern, found keyword `as`
  // Hint: 7-9 keyword `as` is not allowed as an identifier; try `as_` instead
  let as = 10
}

--- markup-expr-incomplete ---
// Error: 2-2 expected expression
#

--- markup-expr-incomplete-followed-by-text ---
// Error: 2-2 expected expression
#  hello
