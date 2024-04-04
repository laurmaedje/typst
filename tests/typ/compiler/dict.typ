// Test dictionaries.

--- dict-basic-syntax ---

// Empty
#(:)

// Two pairs and string key.
#let dict = (normal: 1, "spacy key": 2)
#dict

#test(dict.normal, 1)
#test(dict.at("spacy key"), 2)

--- dict-at-lvalue ---
// Test lvalue and rvalue access.
#{
  let dict = (a: 1, "b b": 1)
  dict.at("b b") += 1
  dict.state = (ok: true, err: false)
  test(dict, (a: 1, "b b": 2, state: (ok: true, err: false)))
  test(dict.state.ok, true)
  dict.at("state").ok = false
  test(dict.state.ok, false)
  test(dict.state.err, false)
}

--- dict-at-missing-key ---
// Test rvalue missing key.
#{
  let dict = (a: 1, b: 2)
  // Error: 11-23 dictionary does not contain key "c" and no default value was specified
  let x = dict.at("c")
}

--- dict-at-default ---
// Test default value.
#test((a: 1, b: 2).at("b", default: 3), 2)
#test((a: 1, b: 2).at("c", default: 3), 3)

--- dict-insert ---
// Test insert.
#{
  let dict = (a: 1, b: 2)
  dict.insert("b", 3)
  test(dict, (a: 1, b: 3))
  dict.insert("c", 5)
  test(dict, (a: 1, b: 3, c: 5))
}

--- dict-remove-with-default ---
// Test remove with default value.
#{
  let dict = (a: 1, b: 2)
  test(dict.remove("b", default: 3), 2)
}

#{
  let dict = (a: 1, b: 2)
  test(dict.remove("c", default: 3), 3)
}

--- dict-missing-lvalue ---
// Missing lvalue is not automatically none-initialized.
#{
  let dict = (:)
  // Error: 3-9 dictionary does not contain key "b"
  // Hint: 3-9 use `insert` to add or update values
  dict.b += 1
}

--- dict-basic-methods ---
// Test dictionary methods.
#let dict = (a: 3, c: 2, b: 1)
#test("c" in dict, true)
#test(dict.len(), 3)
#test(dict.values(), (3, 2, 1))
#test(dict.pairs().map(p => p.first() + str(p.last())).join(), "a3c2b1")

#dict.remove("c")
#test("c" in dict, false)
#test(dict, (a: 3, b: 1))

--- dict-from-module ---
// Test dictionary constructor
#dictionary(sys).at("version")
#dictionary(sys).at("no_crash", default: none)

--- dict-remove-order ---
// Test that removal keeps order.
#let dict = (a: 1, b: 2, c: 3, d: 4)
#dict.remove("b")
#test(dict.keys(), ("a", "c", "d"))

--- dict-duplicate-key ---
// Error: 24-29 duplicate key: first
#(first: 1, second: 2, first: 3)

--- dict-duplicate-key-stringy ---
// Error: 17-20 duplicate key: a
#(a: 1, "b": 2, "a": 3)

--- dict-bad-expression ---
// Simple expression after already being identified as a dictionary.
// Error: 9-10 expected named or keyed pair, found identifier
#(a: 1, b)

--- dict-leading-colon ---
// Identified as dictionary due to initial colon.
// The boolean key is allowed for now since it will only cause an error at the evaluation stage.
// Error: 4-5 expected named or keyed pair, found integer
// Error: 17 expected expression
#(:1 b:"", true:)

--- dict-temporary-lvalue ---
// Error: 3-15 cannot mutate a temporary value
#((key: "val").other = "some")

--- dict-function-item-not-a-method ---
#{
  let dict = (
    call-me: () => 1,
  )
  // Error: 8-15 type dictionary has no method `call-me`
  // Hint: 8-15 to call the function stored in the dictionary, surround the field access with parentheses, e.g. `(dict.call-me)(..)`
  dict.call-me()
}

--- dict-item-missing-method ---
#{
  let dict = (
    nonfunc: 1
  )

  // Error: 8-15 type dictionary has no method `nonfunc`
  // Hint: 8-15 did you mean to access the field `nonfunc`?
  dict.nonfunc()
}

--- dict-dynamic-uplicate-key ---
#let a = "hello"
#let b = "world"
#let c = "value"
#let d = "conflict"

#assert.eq(((a): b), ("hello": "world"))
#assert.eq(((a): 1, (a): 2), ("hello": 2))
#assert.eq((hello: 1, (a): 2), ("hello": 2))
#assert.eq((a + b: c, (a + b): d, (a): "value2", a: "value3"), ("helloworld": "conflict", "hello": "value2", "a": "value3"))

--- dict-bad-destructuring ---
// Error: 7-10 expected identifier, found group
// Error: 12-14 expected pattern, found integer
#let ((a): 10) = "world"

--- dict-bad-key ---
// Error: 3-7 expected string, found boolean
// Error: 16-18 expected string, found integer
#(true: false, 42: 3)
