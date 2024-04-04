// Smart quotes were not appearing in the PDF outline, because they didn't
// implement `PlainText`
// https://github.com/typst/typst/issues/3662

--- issue-3662-pdf-smartquotes ---
= It's "Unnormal Heading"
= It’s “Normal Heading”

#set smartquote(enabled: false)
= It's "Unnormal Heading"
= It's 'single quotes'
= It’s “Normal Heading”
