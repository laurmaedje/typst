// Test setting custom smartquotes

--- smartquote-custom ---
// Use language quotes for missing keys, allow partial reset
#set smartquote(quotes: "«»")
"Double and 'Single' Quotes"

#set smartquote(quotes: (double: auto, single: "«»"))
"Double and 'Single' Quotes"

--- smartquote-custom-complex ---
// Allow 2 graphemes
#set smartquote(quotes: "a\u{0301}a\u{0301}")
"Double and 'Single' Quotes"

#set smartquote(quotes: (single: "a\u{0301}a\u{0301}"))
"Double and 'Single' Quotes"

--- smart-quote-custom-bad-string ---
// Error: 25-28 expected 2 characters, found 1 character
#set smartquote(quotes: "'")

--- smart-quote-custom-bad-array ---
// Error: 25-35 expected 2 quotes, found 4 quotes
#set smartquote(quotes: ("'",) * 4)

--- smart-quote-custom-bad-dict ---
// Error: 25-45 expected 2 quotes, found 4 quotes
#set smartquote(quotes: (single: ("'",) * 4))
