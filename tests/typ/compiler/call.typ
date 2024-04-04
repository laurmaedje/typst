// Test function calls.

--- call-basic ---

// Omitted space.
#let f() = {}
#[#f()*Bold*]

// Call return value of function with body.
#let f(x, body) = (y) => [#x] + body + [#y]
#f(1)[2](3)

// Don't parse this as a function.
#test (it)

#let f(body) = body
#f[A]
#f()[A]
#f([A])

#let g(a, b) = a + b
#g[A][B]
#g([A], [B])
#g()[A][B]

--- args-trailing-comma ---
// Trailing comma.
#test(1 + 1, 2,)

--- call-aliased-function ---
// Call function assigned to variable.
#let alias = type
#test(alias(alias), type)

--- call-complex-callee-expression ---
// Callee expressions.
#{
  // Wrapped in parens.
  test((type)("hi"), str)

  // Call the return value of a function.
  let adder(dx) = x => x + dx
  test(adder(2)(5), 7)
}

--- args-duplicate ---
// Error: 26-30 duplicate argument: font
#set text(font: "Arial", font: "Helvetica")

--- args-bad-positional-as-named ---
// Error: 4-15 the argument `amount` is positional
// Hint: 4-15 try removing `amount:`
#h(amount: 0.5)

--- call-bad-type-bool-literal ---
// Error: 2-6 expected function, found boolean
#true()

--- call-bad-type-string-var ---
#let x = "x"

// Error: 2-3 expected function, found string
#x()

--- call-bad-type-int-expr ---
#let f(x) = x

// Error: 2-6 expected function, found integer
#f(1)(2)

--- call-bad-type-content-expr ---
#let f(x) = x

// Error: 2-6 expected function, found content
#f[1](2)

--- args-bad-colon ---
// Error: 7-8 unexpected colon
#func(:)

--- args-bad-token ---
// Error: 10-12 unexpected end of block comment
#func(a:1*/)

--- args-missing-comma ---
// Error: 8 expected comma
#func(1 2)

--- args-bad-name-and-incomplete-pair ---
// Error: 7-8 expected identifier, found integer
// Error: 9 expected expression
#func(1:)

--- args-bad-name-int ---
// Error: 7-8 expected identifier, found integer
#func(1:2)

--- args-bad-name-string ---
// Error: 7-12 expected identifier, found string
#func("abc": 2)

--- args-bad-name-group ---
// Error: 7-10 expected identifier, found group
#func((x):1)

--- args-content-block-unclosed ---
// Error: 6-7 unclosed delimiter
#func[`a]`

--- args-unclosed ---
// Error: 7-8 unclosed delimiter
#{func(}

--- args-unclosed-string ---
// Error: 6-7 unclosed delimiter
// Error: 1:7-2:1 unclosed string
#func("]
