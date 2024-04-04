// Test references to footnotes.

--- footnote-ref ---
A footnote #footnote[Hi]<fn> \
A reference to it @fn

--- footnote-ref-multiple ---
// Multiple footnotes are refs
First #footnote[A]<fn1> \
Second #footnote[B]<fn2> \
First ref @fn1 \
Third #footnote[C] \
Fourth #footnote[D]<fn4> \
Fourth ref @fn4 \
Second ref @fn2 \
Second ref again @fn2

--- footnote-ref-forward ---
// Forward reference
Usage @fn \
Definition #footnote[Hi]<fn>

--- footnote-ref-in-footnote ---
// Footnote ref in footnote
#footnote[Reference to next @fn]
#footnote[Reference to myself @fn]<fn>
#footnote[Reference to previous @fn]

--- footnote-styling ---
// Styling
#show footnote: text.with(fill: red)
Real #footnote[...]<fn> \
Ref @fn

--- footnote-ref-call ---
// Footnote call with label
#footnote(<fn>)
#footnote[Hi]<fn>
#ref(<fn>)
#footnote(<fn>)
