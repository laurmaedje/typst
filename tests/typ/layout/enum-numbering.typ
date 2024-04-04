// Test enum numbering styles.

--- enum-numbering-pattern ---
// Test numbering pattern.
#set enum(numbering: "(1.a.*)")
+ First
+ Second
  2. Nested
     + Deep
+ Normal

--- enum-numbering-full ---
// Test full numbering.
#set enum(numbering: "1.a.", full: true)
+ First
  + Nested

--- enum-numbering-closure ---
// Test numbering with closure.
#enum(
  start: 3,
  spacing: 0.65em - 3pt,
  tight: false,
  numbering: n => text(
    fill: (red, green, blue).at(calc.rem(n, 3)),
    numbering("A", n),
  ),
  [Red], [Green], [Blue], [Red],
)

--- enum-numbering-closure-nested ---
// Test numbering with closure and nested lists.
#set enum(numbering: n => super[#n])
+ A
  + B
+ C

--- enum-numbering-closure-nested-complex ---
// Test numbering with closure and nested lists.
#set text(font: "New Computer Modern")
#set enum(numbering: (..args) => math.mat(args.pos()), full: true)
+ A
  + B
  + C
    + D
+ E
+ F

--- enum-numbering-pattern-empty ---
// Error: 22-24 invalid numbering pattern
#set enum(numbering: "")

--- enum-numbering-pattern-invalid ---
// Error: 22-28 invalid numbering pattern
#set enum(numbering: "(())")
