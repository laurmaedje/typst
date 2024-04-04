// Test bugs with argument sinks.

--- issue-886-args-sink ---
#let foo(..body) = repr(body.pos())
#foo(a: "1", b: "2", 1, 2, 3, 4, 5, 6)
