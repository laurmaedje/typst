// Test invalid operations.

--- ops-unary-minus-missing-expr ---
// Error: 4 expected expression
#(-)

--- ops-add-missing-rhs ---
// Error: 10 expected expression
#test({1+}, 1)

--- ops-mul-missing-rhs ---
// Error: 10 expected expression
#test({2*}, 2)

--- ops-unary-plus-on-content ---
// Error: 3-13 cannot apply unary '+' to content
#(+([] + []))

--- ops-unary-plus-on-string ---
// Error: 3-6 cannot apply '-' to string
#(-"")

--- ops-not-on-array ---
// Error: 3-9 cannot apply 'not' to array
#(not ())

--- ops-compare-relative-length-and-ratio ---
// Error: 3-19 cannot compare relative length and ratio
#(30% + 1pt <= 40%)

--- ops-compare-em-with-abs ---
// Error: 3-14 cannot compare 1em with 10pt
#(1em <= 10pt)

--- ops-compare-normal-float-with-nan ---
// Error: 3-22 cannot compare 2.2 with NaN
#(2.2 <= float("nan"))

--- ops-compare-int-and-str ---
// Error: 3-26 cannot compare integer and string
#((0, 1, 3) > (0, 1, "a"))

--- ops-compare-array-nested-failure ---
// Error: 3-42 cannot compare 3.5 with NaN
#((0, "a", 3.5) <= (0, "a", float("nan")))

--- ops-divide-by-zero-float ---
// Error: 3-12 cannot divide by zero
#(1.2 / 0.0)

--- ops-divide-by-zero-int ---
// Error: 3-8 cannot divide by zero
#(1 / 0)

--- ops-divide-by-zero-angle ---
// Error: 3-15 cannot divide by zero
#(15deg / 0deg)

--- ops-binary-arithmetic-error-message ---
// Special messages for +, -, * and /.
// Error: 3-10 cannot add integer and string
#(1 + "2", 40% - 1)

--- add-assign-int-and-str ---
// Error: 15-23 cannot add integer and string
#{ let x = 1; x += "2" }

--- ops-divide-ratio-by-length ---
// Error: 4-13 cannot divide ratio by length
#( 10% / 5pt )

--- ops-divide-em-by-abs ---
// Error: 3-12 cannot divide these two lengths
#(1em / 5pt)

--- ops-divide-relative-length-by-ratio ---
// Error: 3-19 cannot divide relative length by ratio
#((10% + 1pt) / 5%)

--- ops-divide-relative-lengths ---
// Error: 3-28 cannot divide these two relative lengths
#((10% + 1pt) / (20% + 1pt))

--- ops-subtract-int-from-ratio ---
// Error: 13-20 cannot subtract integer from ratio
#((1234567, 40% - 1))

--- ops-multiply-int-with-bool ---
// Error: 3-11 cannot multiply integer with boolean
#(2 * true)

--- ops-divide-int-by-length ---
// Error: 3-11 cannot divide integer by length
#(3 / 12pt)

--- multiply-negative-int-with-str ---
// Error: 3-10 number must be at least zero
#(-1 * "")

--- assign-unknown-parenthesized-variable ---
// Error: 4-5 unknown variable: x
#((x) = "")

--- assign-destructuring-unknown-variable ---
// Error: 4-5 unknown variable: x
#((x,) = (1,))

--- assign-to-temporary ---
// Error: 3-8 cannot mutate a temporary value
#(1 + 2 += 3)

--- assign-to-invalid-unary-op ---
// Error: 2:3-2:8 cannot apply 'not' to string
#let x = "Hey"
#(not x = "a")

--- assign-to-invalid-binary-op ---
// Error: 7-8 unknown variable: x
#(1 + x += 3)

--- assign-unknown-variable ---
// Error: 3-4 unknown variable: z
#(z = 1)

--- assign-to-std-constant ---
// Error: 3-7 cannot mutate a constant: rect
#(rect = "hi")

--- assign-to-shadowed-std-constant ---
// Works if we define rect beforehand
// (since then it doesn't resolve to the standard library version anymore).
#let rect = ""
#(rect = "hi")
