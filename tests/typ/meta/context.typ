// Test context expressions.

--- context-body-atomic-in-markup ---
// Test that context body is parsed as atomic expression.
#let c = [#context "hello".]
#test(c.children.first().func(), (context none).func())
#test(c.children.last(), [.])

--- context-element-constructor-forbidden ---
// Test that manual construction is forbidden.
// Error: 2-25 cannot be constructed manually
#(context none).func()()

--- context-query-here ---
// Test that `here()` yields the context element's location.
#context test(query(here()).first().func(), (context none).func())

--- context-get-in-function ---
// Test whether context is retained in nested function.
#let translate(..args) = args.named().at(text.lang)
#set text(lang: "de")
#context test(translate(de: "Inhalt", en: "Contents"), "Inhalt")

--- context-get-in-array-callback ---
// Test whether context is retained in built-in callback.
#set text(lang: "de")
#context test(
  ("en", "de", "fr").sorted(key: v => v != text.lang),
  ("de", "en", "fr"),
)

--- here-position ---
// Test `context` + `here`.
#context test(here().position().y, 10pt)

--- locate-position ---
// Test `locate`.
#v(10pt)
= Introduction <intro>
#context test(locate(<intro>).position().y, 20pt)

--- locate-missing-label ---
// Error: 10-25 label `<intro>` does not exist in the document
#context locate(<intro>)

--- locate-duplicate-label ---
= Introduction <intro>
= Introduction <intro>

// Error: 10-25 label `<intro>` occurs multiple times in the document
#context locate(<intro>)

--- locate-element-selector ---
#v(10pt)
= Introduction <intro>
#context test(locate(heading).position().y, 20pt)

--- locate-element-selector-no-match ---
// Error: 10-25 selector does not match any element
#context locate(heading)

--- locate-element-selector-multiple-matches ---
= Introduction <intro>
= Introduction <intro>

// Error: 10-25 selector matches multiple elements
#context locate(heading)

--- counter-basic-2 ---
// Test `counter`.
#let c = counter("heading")
#c.update(2)
#c.update(n => n + 2)
#context test(c.get(), (4,))
#c.update(n => n - 3)
#context test(c.at(here()), (1,))

--- state-at-no-context ---
// Test `state.at` outside of context.
// Error: 2-26 can only be used when context is known
// Hint: 2-26 try wrapping this in a `context` expression
// Hint: 2-26 the `context` expression should wrap everything that depends on this function
#state("key").at(<label>)

--- counter-at-no-context ---
// Test `counter.at` outside of context.
// Error: 2-28 can only be used when context is known
// Hint: 2-28 try wrapping this in a `context` expression
// Hint: 2-28 the `context` expression should wrap everything that depends on this function
#counter("key").at(<label>)

--- measure ---
// Test `measure`.
#let f(lo, hi) = context {
  let h = measure[Hello].height
  assert(h > lo)
  assert(h < hi)
}
#text(10pt, f(6pt, 8pt))
#text(20pt, f(13pt, 14pt))

--- get-rule-basic ---
// Test basic get rule.
#context test(text.lang, "en")
#set text(lang: "de")
#context test(text.lang, "de")
#text(lang: "es", context test(text.lang, "es"))

--- get-rule-folding ---
// Test folding.
#set rect(stroke: red)
#context {
  test(type(rect.stroke), stroke)
  test(rect.stroke.paint, red)
}
#[
  #set rect(stroke: 4pt)
  #context test(rect.stroke, 4pt + red)
]
#context test(rect.stroke, stroke(red))

--- get-rule-figure-caption-collision ---
// We have one collision: `figure.caption` could be both the element and a get
// rule for the `caption` field, which is settable. We always prefer the
// element. It's unfortunate, but probably nobody writes
// `set figure(caption: ..)` anyway.
#test(type(figure.caption), function)
#context test(type(figure.caption), function)

--- get-rule-assertion-failure ---
// Error: 10-31 Assertion failed: "en" != "de"
#context test(text.lang, "de")

--- get-rule-unknown-field ---
// Error: 15-20 function `text` does not contain field `langs`
#context text.langs

--- get-rule-inherent-field ---
// Error: 18-22 function `heading` does not contain field `body`
#context heading.body

--- get-rule-missing-context-no-context ---
// Error: 7-11 can only be used when context is known
// Hint: 7-11 try wrapping this in a `context` expression
// Hint: 7-11 the `context` expression should wrap everything that depends on this function
#text.lang

--- get-rule-unknown-field-no-context ---
// Error: 7-12 function `text` does not contain field `langs`
#text.langs

--- get-rule-inherent-field-no-context ---
// Error: 10-14 function `heading` does not contain field `body`
#heading.body

--- context-in-show-rule ---
// Test that show rule establishes context.
#set heading(numbering: "1.")
#show heading: it => test(
  counter(heading).get(),
  (intro: (1,), back: (2,)).at(str(it.label)),
)

= Introduction <intro>
= Background <back>

--- context-in-show-rule-query ---
// Test that show rule on non-locatable element allows `query`.
// Error: 18-47 Assertion failed: 2 != 3
#show emph: _ => test(query(heading).len(), 3)
#show strong: _ => test(query(heading).len(), 2)
= Introduction
= Background
*Hi* _there_

--- context-assign-to-captured-variable ---
// Test error when captured variable is assigned to.
#let i = 0
// Error: 11-12 variables from outside the context expression are read-only and cannot be modified
#context (i = 1)
