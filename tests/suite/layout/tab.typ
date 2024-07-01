--- tab ---
A #tab() CD #tab() E \
AB #tab() C #tab() F

--- tab-stops ---
#set tab(stops: 4em)
A #tab() CD #tab() E \
AB #tab() C #tab() F

#set tab(stops: (1cm, 0.5cm))
| #tab() | #tab() | #tab() | #tab() |
| #tab() | #tab() | #tab() | #tab() |

--- tab-fr ---
// Tab before fr is fine, but tab after fr is disabled
A #h(1fr) D \
A #tab() BC #h(1fr) D \
#h(1fr) A #tab() D

--- tab-justify ---
// Test how tab stops and justification interact.
#set par(justify: true)
A B #tab() C D #linebreak(justify: true)

--- tab-bidi ---
// Test tab stops in RTL and mixed text.
#set text(lang: "he")
#set tab(stops: 4em)

#let a = [טֶקסט]
#let b = [טֶט]
#a #tab() #a \
#a #tab() #b

--- tab-list ---
#show enum: it => pad(left: 2em, stack(
  dir: ttb,
  spacing: par.spacing,
  ..it.children.map(item => [
    #h(-2em) #tab(align: end) #item.number. #tab() #item.body
  ])
))

1. #lorem(10)

10. #text(15pt)[This] is some more text for a second item.

1000. #lorem(4)

100000. #lorem(8)
