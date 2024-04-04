// Test show rules.

--- show-selector-basic ---
// Override lists.
#show list: it => "(" + it.children.map(v => v.body).join(", ") + ")"

- A
  - B
  - C
- D
- E

--- show-selector-replace-and-show-set ---
// Test full reset.
#show heading: [B]
#show heading: set text(size: 10pt, weight: 400)
A #[= Heading] C

--- show-selector-discard ---
// Test full removal.
#show heading: none

Where is
= There are no headings around here!
my heading?

--- show-selector-realistic ---
// Test integrated example.
#show heading: it => block({
  set text(10pt)
  box(move(dy: -1pt)[📖])
  h(5pt)
  if it.level == 1 {
    underline(text(1.25em, blue, it.body))
  } else {
    text(red, it.body)
  }
})

= Task 1
Some text.

== Subtask
Some more text.

= Task 2
Another text.

--- show-in-show ---
// Test set and show in code blocks.
#show heading: it => {
  set text(red)
  show "ding": [🛎]
  it.body
}

= Heading

--- show-nested-scopes ---
// Test that scoping works as expected.
#{
  let world = [ World ]
  show "W": strong
  world
  {
    set text(blue)
    show: it => {
      show "o": "Ø"
      it
    }
    world
  }
  world
}

--- show-selector-replace ---
#show heading: [1234]
= Heading

--- show-unknown-field ---
// Error: 25-29 content does not contain field "page"
#show heading: it => it.page
= Heading

--- show-text-element-discard ---
#show text: none
Hey

--- show-selector-not-an-element-function ---
// Error: 7-12 only element functions can be used as selectors
#show upper: it => {}

--- show-bad-replacement-type ---
// Error: 16-20 expected content or function, found integer
#show heading: 1234
= Heading

--- show-bad-selector-type ---
// Error: 7-10 expected symbol, string, label, function, regex, or selector, found color
#show red: []

--- show-selector-in-expression ---
// Error: 7-25 show is only allowed directly in code and content blocks
#(1 + show heading: none)
