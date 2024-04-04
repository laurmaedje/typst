// Test tracking characters apart or together.

--- text-tracking-negative ---
// Test tracking.
#set text(tracking: -0.01em)
I saw Zoe yӛsterday, on the tram.

--- text-tracking-changed-temporarily ---
// Test tracking for only part of paragraph.
I'm in#text(tracking: 0.15em + 1.5pt)[ spaace]!

--- text-tracking-mark-placement ---
// Test that tracking doesn't disrupt mark placement.
#set text(font: ("PT Sans", "Noto Serif Hebrew"))
#set text(tracking: 0.3em)
טֶקסט

--- text-tracking-arabic ---
// Test tracking in arabic text (makes no sense whatsoever)
#set text(tracking: 0.3em)
النص

--- text-spacing ---
// Test word spacing.
#set text(spacing: 1em)
My text has spaces.

--- text-spacing-relative ---
// Test word spacing relative to the font's space width.
#set text(spacing: 50% + 1pt)
This is tight.
