// Test let bindings.

--- let-basic ---
// Automatically initialized with none.
#let x
#test(x, none)

// Manually initialized with one.
#let z = 1
#test(z, 1)

// Syntax sugar for function definitions.
#let fill = conifer
#let f(body) = rect(width: 2cm, fill: fill, inset: 5pt, body)
#f[Hi!]

--- let-termination ---
// Termination.

// Terminated by line break.
#let v1 = 1
One

// Terminated by semicolon.
#let v2 = 2; Two

// Terminated by semicolon and line break.
#let v3 = 3;
Three

#test(v1, 1)
#test(v2, 2)
#test(v3, 3)

--- let-valid-idents ---
// Test what constitutes a valid Typst identifier.
#let name = 1
#test(name, 1)
#let name_ = 1
#test(name_, 1)
#let name-2 = 1
#test(name-2, 1)
#let name_2 = 1
#test(name_2, 1)
#let __name = 1
#test(__name, 1)
#let ůñıćóðė = 1
#test(ůñıćóðė, 1)

--- let-ident-parenthesized ---
// Test parenthesised assignments.
#let (a) = (1, 2)

--- let-destructuring-array ---
// Simple destructuring.
#let (a, b) = (1, 2)
#test(a, 1)
#test(b, 2)

--- let-destructuring-array-single-item ---
#let (a,) = (1,)
#test(a, 1)

--- let-destructuring-array-placeholders ---
// Destructuring with multiple placeholders.
#let (a, _, c, _) = (1, 2, 3, 4)
#test(a, 1)
#test(c, 3)

--- let-destructuring-array-with-sink-at-end ---
// Destructuring with a sink.
#let (a, b, ..c) = (1, 2, 3, 4, 5, 6)
#test(a, 1)
#test(b, 2)
#test(c, (3, 4, 5, 6))

--- let-destructuring-array-with-sink-in-middle ---
// Destructuring with a sink in the middle.
#let (a, ..b, c) = (1, 2, 3, 4, 5, 6)
#test(a, 1)
#test(b, (2, 3, 4, 5))
#test(c, 6)

--- let-destructuring-array-with-sink-at-start-empty ---
// Destructuring with an empty sink.
#let (..a, b, c) = (1, 2)
#test(a, ())
#test(b, 1)
#test(c, 2)

--- let-destructuring-array-with-sink-in-middle-empty ---
// Destructuring with an empty sink.
#let (a, ..b, c) = (1, 2)
#test(a, 1)
#test(b, ())
#test(c, 2)

--- let-destructuring-array-with-sink-at-end-empty ---
// Destructuring with an empty sink.
#let (a, b, ..c) = (1, 2)
#test(a, 1)
#test(b, 2)
#test(c, ())

--- let-destructuring-array-with-sink-empty ---
// Destructuring with an empty sink and empty array.
#let (..a) = ()
#test(a, ())

--- let-destructuring-array-with-unnamed-sink ---
// Destructuring with unnamed sink.
#let (a, .., b) = (1, 2, 3, 4)
#test(a, 1)
#test(b, 4)

// Error: 10-11 duplicate binding: a
#let (a, a) = (1, 2)

// Error: 12-15 only one destructuring sink is allowed
#let (..a, ..a) = (1, 2)

// Error: 12-13 duplicate binding: a
#let (a, ..a) = (1, 2)

// Error: 13-14 duplicate binding: a
#let (a: a, a) = (a: 1, b: 2)

// Error: 13-20 expected pattern, found function call
#let (a, b: b.at(0)) = (a: 1, b: 2)

// Error: 7-14 expected pattern, found function call
#let (a.at(0),) = (1,)

--- let-destructuring-array-too-few-elements ---
// Error: 13-14 not enough elements to destructure
#let (a, b, c) = (1, 2)

--- let-destructuring-array-too-few-elements-with-sink ---
// Error: 7-10 not enough elements to destructure
#let (..a, b, c, d) = (1, 2)

--- let-destructuring-array-bool-invalid ---
// Error: 6-12 cannot destructure boolean
#let (a, b) = true

--- let-destructuring-dict ---
// Simple destructuring.
#let (a: a, b, x: c) = (a: 1, b: 2, x: 3)
#test(a, 1)
#test(b, 2)
#test(c, 3)

--- let-destructuring-dict-with-sink-at-end ---
// Destructuring with a sink.
#let (a: _, ..b) = (a: 1, b: 2, c: 3)
#test(b, (b: 2, c: 3))

--- let-destructuring-dict-with-sink-in-middle ---
// Destructuring with a sink in the middle.
#let (a: _, ..b, c: _) = (a: 1, b: 2, c: 3)
#test(b, (b: 2))

--- let-destructuring-dict-with-sink-at-end-empty ---
// Destructuring with an empty sink.
#let (a: _, ..b) = (a: 1)
#test(b, (:))

--- let-destructuring-dict-with-sink-empty ---
// Destructuring with an empty sink and empty dict.
#let (..a) = (:)
#test(a, (:))

--- let-destructuring-dict-with-unnamed-sink ---
// Destructuring with unnamed sink.
#let (a, ..) = (a: 1, b: 2)
#test(a, 1)

--- let-destructuring-nested ---
// Nested destructuring.
#let ((a, b), (key: c)) = ((1, 2), (key: 3))
#test((a, b, c), (1, 2, 3))

--- let-destructuring-dict-key-string-invalid ---
// Keyed destructuring is not currently supported.
// Error: 7-18 expected pattern, found string
#let ("spacy key": val) = ("spacy key": 123)
#val

--- let-destructuring-dict-key-expr-invalid ---
// Keyed destructuring is not currently supported.
#let x = "spacy key"
// Error: 7-10 expected identifier, found group
#let ((x): v) = ("spacy key": 123)

--- let-destructuring-array-trailing-placeholders ---
// Trailing placeholders.
// Error: 10-11 not enough elements to destructure
#let (a, _, _, _, _) = (1,)
#test(a, 1)

--- let-destructuring-dict-patterns-invalid ---
// Error: 10-13 expected pattern, found string
// Error: 18-19 expected pattern, found integer
#let (a: "a", b: 2) = (a: 1, b: 2)

--- let-destructuring-dict-shorthand-missing-key ---
// Error: 10-11 dictionary does not contain key "b"
#let (a, b) = (a: 1)

--- let-destructuring-dict-missing-key ---
// Error: 10-11 dictionary does not contain key "b"
#let (a, b: b) = (a: 1)

--- let-destructuring-dict-from-array ---
// Error: 7-11 cannot destructure named pattern from an array
#let (a: a, b) = (1, 2, 3)

--- let-incomplete ---
// Error: 5 expected pattern
#let

// Error: 6 expected pattern
#{let}

// Error: 6-9 expected pattern, found string
#let "v"

// Error: 7 expected semicolon or line break
#let v 1

// Error: 9 expected expression
#let v =

// Error: 6-9 expected pattern, found string
#let "v" = 1

// Terminated because expression ends.
// Error: 12 expected semicolon or line break
#let v4 = 4 Four

// Terminated by semicolon even though we are in a paren group.
// Error: 18 expected expression
// Error: 11-12 unclosed delimiter
#let v5 = (1, 2 + ; Five

// Error: 9-13 expected pattern, found boolean
#let (..true) = false

--- underscore-invalid ---
#let _ = 4

#for _ in range(2) []

// Error: 2-3 unexpected underscore
#_

// Error: 8-9 expected expression, found underscore
#lorem(_)

// Error: 3-4 expected expression, found underscore
#(_,)

// Error: 3-4 expected expression, found underscore
#{_}

// Error: 8-9 expected expression, found underscore
#{ 1 + _ }

--- let-function-incomplete ---
// Error: 13 expected equals sign
#let func(x)

// Error: 15 expected expression
#let func(x) =

--- let-function-parenthesized ---
// This is not yet parsed in the ideal way.
// Error: 12 expected equals sign
#let (func)(x)

--- let-function-parenthesized-with-init ---
// These errors aren't great.
// Error: 12 expected equals sign
// Error: 15-15 expected semicolon or line break
#let (func)(x) = 3
