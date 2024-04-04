// Big numbers (larger than what i64 can store) should just lose some precision
// but not overflow
// https://github.com/typst/typst/issues/3363

--- issue-3363-json-large-number ---
#let bignum = json("/assets/data/big-number.json")

#bignum
