// Issue #3154: Confusing errors from methods supposed to return a mutable entry
// https://github.com/typst/typst/issues/3154

--- issue-3154-array-first-empty ---
#{
  let array = ()
  // Error: 3-16 array is empty
  array.first()
}

--- issue-3154-array-first-mutable-empty ---
#{
  let array = ()
  // Error: 3-16 array is empty
  array.first() = 9
}

--- issue-3154-array-last-empty ---
#{
  let array = ()
  // Error: 3-15 array is empty
  array.last()
}

--- issue-3154-array-last-mutable-empty ---
#{
  let array = ()
  // Error: 3-15 array is empty
  array.last() = 9
}

--- issue-3154-array-at-out-of-bounds ---
#{
  let array = (1,)
  // Error: 3-14 array index out of bounds (index: 1, len: 1) and no default value was specified
  array.at(1)
}

--- issue-3154-array-at-out-of-bounds-default ---
#{
  let array = (1,)
  test(array.at(1, default: 0), 0)
}

--- issue-3154-array-at-out-of-bounds-mutable ---
#{
  let array = (1,)
  // Error: 3-14 array index out of bounds (index: 1, len: 1)
  array.at(1) = 9
}

--- issue-3154-array-at-out-of-bounds-mutable-default ---
#{
  let array = (1,)
  // Error: 3-26 array index out of bounds (index: 1, len: 1)
  array.at(1, default: 0) = 9
}

--- issue-3154-dict-at-not-contained ---
#{
  let dict = (a: 1)
  // Error: 3-15 dictionary does not contain key "b" and no default value was specified
  dict.at("b")
}

--- issue-3154-dict-at-missing-default ---
#{
  let dict = (a: 1)
  test(dict.at("b", default: 0), 0)
}

--- issue-3154-dict-at-missing-mutable ---
#{
  let dict = (a: 1)
  // Error: 3-15 dictionary does not contain key "b"
  // Hint: 3-15 use `insert` to add or update values
  dict.at("b") = 9
}

--- issue-3154-dict-at-missing-mutable-default ---
#{
  let dict = (a: 1)
  // Error: 3-27 dictionary does not contain key "b"
  // Hint: 3-27 use `insert` to add or update values
  dict.at("b", default: 0) = 9
}

--- issue-3154-dict-syntax-missing ---
#{
  let dict = (a: 1)
  // Error: 8-9 dictionary does not contain key "b"
  dict.b
}

--- issue-3154-dict-syntax-missing-mutable ---
#{
  let dict = (a: 1)
  dict.b = 9
  test(dict, (a: 1, b: 9))
}

--- issue-3154-dict-syntax-missing-add-assign ---
#{
  let dict = (a: 1)
  // Error: 3-9 dictionary does not contain key "b"
  // Hint: 3-9 use `insert` to add or update values
  dict.b += 9
}
