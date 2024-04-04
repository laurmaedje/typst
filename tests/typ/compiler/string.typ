// Test the string methods.

--- string-len ---
// Test the `len` method.
#test("Hello World!".len(), 12)

--- string-first-and-last ---
// Test the `first` and `last` methods.
#test("Hello".first(), "H")
#test("Hello".last(), "o")
#test("🏳️‍🌈A🏳️‍⚧️".first(), "🏳️‍🌈")
#test("🏳️‍🌈A🏳️‍⚧️".last(), "🏳️‍⚧️")

--- string-first-empty ---
// Error: 2-12 string is empty
#"".first()

--- string-last-empty ---
// Error: 2-11 string is empty
#"".last()

--- string-at ---
// Test the `at` method.
#test("Hello".at(1), "e")
#test("Hello".at(4), "o")
#test("Hello".at(-1), "o")
#test("Hello".at(-2), "l")
#test("Hey: 🏳️‍🌈 there!".at(5), "🏳️‍🌈")

--- string-at-default ---
// Test `at`'s 'default' parameter.
#test("z", "Hello".at(5, default: "z"))

--- string-at-not-a-char-boundary ---
// Error: 2-14 string index 2 is not a character boundary
#"🏳️‍🌈".at(2)

--- string-at-out-of-bounds ---
// Error: 2-15 no default value was specified and string index out of bounds (index: 5, len: 5)
#"Hello".at(5)

--- string-at-at-default-other-type ---
#test("Hello".at(5, default: (a: 10)), (a: 10))

--- string-slice ---
// Test the `slice` method.
#test("abc".slice(1, 2), "b")
#test("abc🏡def".slice(2, 7), "c🏡")
#test("abc🏡def".slice(2, -2), "c🏡d")
#test("abc🏡def".slice(-3, -1), "de")

--- string-slice-not-a-char-boundary ---
// Error: 2-21 string index -1 is not a character boundary
#"🏳️‍🌈".slice(0, -1)

--- string-clusters ---
// Test the `clusters` and `codepoints` methods.
#test("abc".clusters(), ("a", "b", "c"))
#test("abc".clusters(), ("a", "b", "c"))
#test("🏳️‍🌈!".clusters(), ("🏳️‍🌈", "!"))

--- string-codepoints ---
#test("🏳️‍🌈!".codepoints(), ("🏳", "\u{fe0f}", "\u{200d}", "🌈", "!"))

--- string-contains ---
// Test the `contains` method.
#test("abc".contains("b"), true)
#test("b" in "abc", true)
#test("1234f".contains(regex("\d")), true)
#test(regex("\d") in "1234f", true)
#test("abc".contains("d"), false)
#test("1234g" in "1234f", false)
#test("abc".contains(regex("^[abc]$")), false)
#test("abc".contains(regex("^[abc]+$")), true)

--- string-starts-with ---
// Test the `starts-with` and `ends-with` methods.
#test("Typst".starts-with("Ty"), true)
#test("Typst".starts-with(regex("[Tt]ys")), false)
#test("Typst".starts-with("st"), false)

--- string-ends-with ---
#test("Typst".ends-with("st"), true)
#test("Typst".ends-with(regex("\d*")), true)
#test("Typst".ends-with(regex("\d+")), false)
#test("Typ12".ends-with(regex("\d+")), true)
#test("typst13".ends-with(regex("1[0-9]")), true)
#test("typst113".ends-with(regex("1[0-9]")), true)
#test("typst23".ends-with(regex("1[0-9]")), false)

--- string-find-and-position ---
// Test the `find` and `position` methods.
#let date = regex("\d{2}:\d{2}")
#test("Hello World".find("World"), "World")
#test("Hello World".position("World"), 6)
#test("It's 12:13 now".find(date), "12:13")
#test("It's 12:13 now".position(date), 5)

--- string-match ---
// Test the `match` method.
#test("Is there a".match("for this?"), none)
#test(
  "The time of my life.".match(regex("[mit]+e")),
  (start: 4, end: 8, text: "time", captures: ()),
)

--- string-matches ---
// Test the `matches` method.
#test("Hello there".matches("\d"), ())
#test("Day by Day.".matches("Day"), (
  (start: 0, end: 3, text: "Day", captures: ()),
  (start: 7, end: 10, text: "Day", captures: ()),
))

// Compute the sum of all timestamps in the text.
#let timesum(text) = {
  let time = 0
  for match in text.matches(regex("(\d+):(\d+)")) {
    let caps = match.captures
    time += 60 * int(caps.at(0)) + int(caps.at(1))
  }
  str(int(time / 60)) + ":" + str(calc.rem(time, 60))
}

#test(timesum(""), "0:0")
#test(timesum("2:70"), "3:10")
#test(timesum("1:20, 2:10, 0:40"), "4:10")

--- stgring-replace ---
// Test the `replace` method with `Str` replacements.
#test("ABC".replace("", "-"), "-A-B-C-")
#test("Ok".replace("Ok", "Nope", count: 0), "Ok")
#test("to add?".replace("", "How ", count: 1), "How to add?")
#test("AB C DEF GH J".replace(" ", ",", count: 2), "AB,C,DEF GH J")
#test("Walcemo"
  .replace("o", "k")
  .replace("e", "o")
  .replace("k", "e")
  .replace("a", "e"),
  "Welcome"
)
#test("123".replace(regex("\d$"), "_"), "12_")
#test("123".replace(regex("\d{1,2}$"), "__"), "1__")

--- string-replace-function ---
// Test the `replace` method with `Func` replacements.

#test("abc".replace(regex("[a-z]"), m => {
  str(m.start) + m.text + str(m.end)
}), "0a11b22c3")
#test("abcd, efgh".replace(regex("\w+"), m => {
  upper(m.text)
}), "ABCD, EFGH")
#test("hello : world".replace(regex("^(.+)\s*(:)\s*(.+)$"), m => {
  upper(m.captures.at(0)) + m.captures.at(1) + " " + upper(m.captures.at(2))
}), "HELLO : WORLD")
#test("hello world, lorem ipsum".replace(regex("(\w+) (\w+)"), m => {
  m.captures.at(1) + " " + m.captures.at(0)
}), "world hello, ipsum lorem")
#test("hello world, lorem ipsum".replace(regex("(\w+) (\w+)"), count: 1, m => {
  m.captures.at(1) + " " + m.captures.at(0)
}), "world hello, lorem ipsum")
#test("123 456".replace(regex("[a-z]+"), "a"), "123 456")

#test("abc".replace("", m => "-"), "-a-b-c-")
#test("abc".replace("", m => "-", count: 1), "-abc")
#test("123".replace("abc", m => ""), "123")
#test("123".replace("abc", m => "", count: 2), "123")
#test("a123b123c".replace("123", m => {
  str(m.start) + "-" + str(m.end)
}), "a1-4b5-8c")
#test("halla warld".replace("a", m => {
  if m.start == 1 { "e" }
  else if m.start == 4 or m.start == 7 { "o" }
}), "hello world")
#test("aaa".replace("a", m => str(m.captures.len())), "000")

--- string-replace-function-bad-type ---
// Error: 23-24 expected string, found integer
#"123".replace("123", m => 1)

--- string-replace-bad-type ---
// Error: 23-32 expected string or function, found array
#"123".replace("123", (1, 2, 3))

--- string-trim-basic ---
// Test the `trim` method; the pattern is not provided.
#let str = "Typst, LaTeX, Word, InDesign"
#let array = ("Typst", "LaTeX", "Word", "InDesign")
#test(str.split(",").map(s => s.trim()), array)
#test("".trim(), "")
#test("  ".trim(), "")
#test("\t".trim(), "")
#test("\n".trim(), "")
#test("\t \n".trim(), "")
#test(" abc ".trim(at: start), "abc ")
#test("\tabc ".trim(at: start), "abc ")
#test("abc\n".trim(at: end), "abc")
#test(" abc ".trim(at: end, repeat: true), " abc")
#test("  abc".trim(at: start, repeat: false), "abc")

--- string-trim-pattern-str ---
// Test the `trim` method; the pattern is a string.
#test("aabcaa".trim("a", repeat: false), "abca")
#test("aabca".trim("a", at: start), "bca")
#test("aabcaa".trim("a", at: end, repeat: false), "aabca")
#test(" abc\n".trim("\n"), " abc")
#test("whole".trim("whole", at: start), "")

--- string-trim-pattern-regex ---
// Test the `trim` method; the pattern is a regex.
#test("".trim(regex(".")), "")
#test("123abc456".trim(regex("\d")), "abc")
#test("123abc456".trim(regex("\d"), repeat: false), "23abc45")
#test("123a4b5c678".trim(regex("\d"), repeat: true), "a4b5c")
#test("123a4b5c678".trim(regex("\d"), repeat: false), "23a4b5c67")
#test("123abc456".trim(regex("\d"), at: start), "abc456")
#test("123abc456".trim(regex("\d"), at: end), "123abc")
#test("123abc456".trim(regex("\d+"), at: end, repeat: false), "123abc")
#test("123abc456".trim(regex("\d{1,2}$"), repeat: false), "123abc4")
#test("hello world".trim(regex(".")), "")
#test("12306".trim(regex("\d"), at: start), "")
#test("12306abc".trim(regex("\d"), at: start), "abc")
#test("whole".trim(regex("whole"), at: start), "")
#test("12306".trim(regex("\d"), at: end), "")
#test("abc12306".trim(regex("\d"), at: end), "abc")
#test("whole".trim(regex("whole"), at: end), "")

--- string-trim-at-bad-alignment ---
// Error: 17-21 expected either `start` or `end`
#"abc".trim(at: left)

--- string-split ---
// Test the `split` method.
#test("abc".split(""), ("", "a", "b", "c", ""))
#test("abc".split("b"), ("a", "c"))
#test("a123c".split(regex("\d")), ("a", "", "", "c"))
#test("a123c".split(regex("\d+")), ("a", "c"))

--- string-rev ---
// Test the `rev` method.
#test("abc".rev(), "cba")
#test("ax̂e".rev(), "ex̂a")

--- string-unclosed ---
// Error: 2-2:1 unclosed string
#"hello\"
