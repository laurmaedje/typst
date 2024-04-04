// Test foundational functions.

--- type ---
#test(type(1), int)
#test(type(ltr), direction)
#test(type(10 / 3), float)

--- repr ---
#test(repr(ltr), "ltr")
#test(repr((1, 2, false, )), "(1, 2, false)")

--- panic ---
// Test panic.
// Error: 2-9 panicked
#panic()

--- panic-with-int ---
// Test panic.
// Error: 2-12 panicked with: 123
#panic(123)

--- panic-with-str ---
// Test panic.
// Error: 2-24 panicked with: "this is wrong"
#panic("this is wrong")

--- assert-fail ---
// Test failing assertions.
// Error: 2-16 assertion failed
#assert(1 == 2)

--- assert-fail-message ---
// Test failing assertions.
// Error: 2-51 assertion failed: two is smaller than one
#assert(2 < 1, message: "two is smaller than one")

--- assert-bad-type ---
// Test failing assertions.
// Error: 9-15 expected boolean, found string
#assert("true")

--- assert-eq-fail ---
// Test failing assertions.
// Error: 2-19 equality assertion failed: value 10 was not equal to 11
#assert.eq(10, 11)

--- assert-eq-fail-message ---
// Test failing assertions.
// Error: 2-55 equality assertion failed: 10 and 12 are not equal
#assert.eq(10, 12, message: "10 and 12 are not equal")

--- assert-ne-fail ---
// Test failing assertions.
// Error: 2-19 inequality assertion failed: value 11 was equal to 11
#assert.ne(11, 11)

--- assert-ne-fail-message ---
// Test failing assertions.
// Error: 2-57 inequality assertion failed: must be different from 11
#assert.ne(11, 11, message: "must be different from 11")

--- assert-success ---
// Test successful assertions.
#assert(5 > 3)
#assert.eq(15, 15)
#assert.ne(10, 12)

--- eval ---
// Test the eval function.
#test(eval("1 + 2"), 3)
#test(eval("1 + x", scope: (x: 3)), 4)
#test(eval("let x = x + 1; x + 1", scope: (x: 1)), 3)

--- eval-mode ---
// Test evaluation in other modes.
#eval("[_Hello" + " World!_]") \
#eval("_Hello" + " World!_", mode: "markup") \
#eval("RR_1^NN", mode: "math", scope: (RR: math.NN, NN: math.RR))

--- eval-syntax-error-1 ---
// Error: 7-12 expected pattern
#eval("let")

--- eval-in-show-rule ---
#show raw: it => text(font: "PT Sans", eval("[" + it.text + "]"))

Interacting
```
#set text(blue)
Blue #move(dy: -0.15em)[🌊]
```

--- eval-runtime-error ---
// Error: 7-17 cannot continue outside of loop
#eval("continue")

--- eval-syntax-error-2 ---
// Error: 7-12 expected semicolon or line break
#eval("1 2")
