// Tests outline 'indent' option.

--- outline-indent-numbering ---
// With heading numbering
#set page(width: 200pt)
#set heading(numbering: "1.a.")
#outline()
#outline(indent: false)
#outline(indent: true)
#outline(indent: none)
#outline(indent: auto)
#outline(indent: 2em)
#outline(indent: n => ([-], [], [==], [====]).at(n))
#outline(indent: n => "!" * calc.pow(2, n))

= About ACME Corp.

== History
#lorem(10)

== Products
#lorem(10)

=== Categories
#lorem(10)

==== General
#lorem(10)

--- outline-indent-no-numbering ---
// Without heading numbering
#set page(width: 200pt)
#outline()
#outline(indent: false)
#outline(indent: true)
#outline(indent: none)
#outline(indent: auto)
#outline(indent: n => 2em * n)
#outline(indent: n => ([-], [], [==], [====]).at(n))
#outline(indent: n => "!" * calc.pow(2, n))

= About ACME Corp.

== History
#lorem(10)

== Products
#lorem(10)

=== Categories
#lorem(10)

==== General
#lorem(10)

--- outline-indent-bad-type ---
// Error: 2-35 expected relative length or content, found dictionary
#outline(indent: n => (a: "dict"))

= Heading
