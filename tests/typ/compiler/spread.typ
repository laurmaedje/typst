// Test argument sinks and spreading.

--- args-spread-override ---
// Test standard argument overriding.
#{
  let f(style: "normal", weight: "regular") = {
    "(style: " + style + ", weight: " + weight + ")"
  }

  let myf(..args) = f(weight: "bold", ..args)
  test(myf(), "(style: normal, weight: bold)")
  test(myf(weight: "black"), "(style: normal, weight: black)")
  test(myf(style: "italic"), "(style: italic, weight: bold)")
}

--- args-spread-forward ---
// Test multiple calls.
#{
  let f(b, c: "!") = b + c
  let g(a, ..sink) = a + f(..sink)
  test(g("a", "b", c: "c"), "abc")
}

--- args-spread-type-repr ---
// Test doing things with arguments.
#{
  let save(..args) = {
    test(type(args), arguments)
    test(repr(args), "(three: true, 1, 2)")
  }

  save(1, 2, three: true)
}

--- args-spread-array-and-dict ---
// Test spreading array and dictionary.
#{
  let more = (3, -3, 6, 10)
  test(calc.min(1, 2, ..more), -3)
  test(calc.max(..more, 9), 10)
  test(calc.max(..more, 11), 11)
}

#{
  let more = (c: 3, d: 4)
  let tostr(..args) = repr(args)
  test(tostr(a: 1, ..more, b: 2), "(a: 1, c: 3, d: 4, b: 2)")
}

--- args-spread-none ---
// None is spreadable.
#let f() = none
#f(..none)
#f(..if false {})
#f(..for x in () [])

--- params-sink-unnamed ---
// unnamed spread
#let f(.., a) = a
#test(f(1, 2, 3), 3)

--- args-spread-string-invalid ---
// Error: 11-19 cannot spread string
#calc.min(.."nope")

--- params-sink-bool-invalid ---
// Error: 10-14 expected pattern, found boolean
#let f(..true) = none

--- params-sink-multiple-invalid ---
// Error: 13-16 only one argument sink is allowed
#let f(..a, ..b) = none

--- spread-into-array ---
// Test spreading into array and dictionary.
#{
  let l = (1, 2, 3)
  let r = (5, 6, 7)
  test((..l, 4, ..r), range(1, 8))
  test((..none), ())
}

--- spread-into-dict ---
#{
  let x = (a: 1)
  let y = (b: 2)
  let z = (a: 3)
  test((:..x, ..y, ..z), (a: 3, b: 2))
  test((..(a: 1), b: 2), (a: 1, b: 2))
}

--- spread-dict-into-array ---
// Error: 9-17 cannot spread dictionary into array
#(1, 2, ..(a: 1))

--- spread-array-into-dict ---
// Error: 3-11 cannot spread array into dictionary
#(..(1, 2), a: 1)

--- params-sink-at-start ---
// Spread at beginning.
#{
  let f(..a, b) = (a, b)
  test(repr(f(1)), "((), 1)")
  test(repr(f(1, 2, 3)), "((1, 2), 3)")
  test(repr(f(1, 2, 3, 4, 5)), "((1, 2, 3, 4), 5)")
}

--- params-sink-in-middle ---
// Spread in the middle.
#{
  let f(a, ..b, c) = (a, b, c)
  test(repr(f(1, 2)), "(1, (), 2)")
  test(repr(f(1, 2, 3, 4, 5)), "(1, (2, 3, 4), 5)")
}

--- params-sink-unnamed-empty ---
// Unnamed sink should just ignore any extra arguments.
#{
  let f(a, b: 5, ..) = (a, b)
  test(f(4), (4, 5))
  test(f(10, b: 11), (10, 11))
  test(f(13, 20, b: 12), (13, 12))
  test(f(15, b: 16, c: 13), (15, 16))
}

--- params-sink-missing-arguments ---
#{
  let f(..a, b, c, d) = none

  // Error: 3-10 missing argument: d
  f(1, 2)
}
