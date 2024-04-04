// Issue #3275: clearer errors for loops, https://github.com/typst/typst/issues/3275

--- issue-3275-normal-variable ---
// Normal variable.
#for x in (1, 2) {}
#for x in (a: 1, b: 2) {}
#for x in "foo" {}
#for x in bytes("😊") {}

--- issue-3275-placeholder ---
// Placeholder.
#for _ in (1, 2) {}
#for _ in (a: 1, b: 2) {}
#for _ in "foo" {}
#for _ in bytes("😊") {}

--- issue-3275-destructuring ---
// Destructuring.
#for (a,b,c) in (("a", 1, bytes(())), ("b", 2, bytes(""))) {}
#for (a, ..) in (("a", 1, bytes(())), ("b", 2, bytes(""))) {}
#for (k, v)  in (a: 1, b: 2, c: 3) {}
#for (.., v) in (a: 1, b: 2, c: 3) {}

--- issue-3275-loop-over-content ---
// Error: 11-17 cannot loop over content
#for x in [1, 2] {}

--- issue-3275-loop-over-arguments ---
// Error: 11-25 cannot loop over arguments
#for _ in arguments("a") {}

--- issue-3275-loop-over-integer ---
// Error: 16-21 cannot loop over integer
#for (x, y) in 12306 {}

--- issue-3275-destructuring-loop-over-content ---
// Error: 16-22 cannot loop over content
#for (x, y) in [1, 2] {}

--- issue-3275-destructuring-loop-over-string ---
// Error: 6-12 cannot destructure values of string
#for (x, y) in "foo" {}

--- issue-3275-destructuring-loop-over-string-array ---
// Error: 6-12 cannot destructure string
#for (x, y) in ("foo", "bar") {}

--- issue-3275-destructuring-loop-over-bytes ---
// Error: 6-12 cannot destructure values of bytes
#for (x, y) in bytes("😊") {}

--- issue-3275-destructuring-loop-over-bytes-array ---
// Error: 6-12 cannot destructure bytes
#for (x, y) in (bytes((1,2)), bytes((1,2))) {}

--- issue-3275-destructuring-loop-over-int-array ---
// Error: 6-12 cannot destructure integer
#for (x, y) in (1, 2) {}

--- issue-3275-destructuring-loop-over-2d-array-1 ---
// Error: 10-11 not enough elements to destructure
#for (x, y) in ((1,), (2,)) {}

--- issue-3275-destructuring-loop-over-2d-array-2 ---
// Error: 6-12 too many elements to destructure
#for (x, y) in ((1,2,3), (4,5,6)) {}
