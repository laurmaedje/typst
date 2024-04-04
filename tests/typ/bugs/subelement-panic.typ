// Test that figure captions don't cause panics.

--- issue-2530-figure-caption-panic ---
// #2530
#figure(caption: [test])[].caption

--- issue-2165-figure-caption-panic ---
// #2165
#figure.caption[]

--- issue-2328-figure-entry-panic ---
// #2328
// Error: 4-43 footnote entry must have a location
// Hint: 4-43 try using a query or a show rule to customize the footnote instead
HI#footnote.entry(clearance: 2.5em)[There]

--- issue-2530-enum-item-panic ---
// Enum item (pre-emptive)
#enum.item(none)[Hello]
#enum.item(17)[Hello]

--- issue-2530-list-item-panic ---
// List item (pre-emptive)
#list.item[Hello]

--- issue-2530-term-item-panic ---
// Term item (pre-emptive)
#terms.item[Hello][World!]

--- issue-2530-outline-entry-panic-text ---
// Outline entry (pre-emptive)
// Error: 2-48 cannot outline text
#outline.entry(1, [Hello], [World!], none, [1])

--- issue-2530-outline-entry-panic-heading ---
// Outline entry (pre-emptive, improved error)
// Error: 2-55 heading must have a location
// Hint: 2-55 try using a query or a show rule to customize the outline.entry instead
#outline.entry(1, heading[Hello], [World!], none, [1])
