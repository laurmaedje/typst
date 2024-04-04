// Test text styling in math.

--- math-style-italic-default ---
// Test italic defaults.
$a, A, delta, ϵ, diff, Delta, ϴ$

--- math-style ---
// Test forcing a specific style.
$A, italic(A), upright(A), bold(A), bold(upright(A)), \
 serif(A), sans(A), cal(A), frak(A), mono(A), bb(A), \
 italic(diff), upright(diff), \
 bb("hello") + bold(cal("world")), \
 mono("SQRT")(x) wreath mono(123 + 456)$

--- math-size ---
// Test forcing math size
$a/b, display(a/b), display(a)/display(b), inline(a/b), script(a/b), sscript(a/b) \
 mono(script(a/b)), script(mono(a/b))\
 script(a^b, cramped: #true), script(a^b, cramped: #false)$

--- math-style-exceptions ---
// Test a few style exceptions.
$h, bb(N), cal(R), Theta, italic(Theta), sans(Theta), sans(italic(Theta))$

--- math-style-hebrew-exceptions ---
// Test hebrew exceptions.
$aleph, beth, gimel, daleth$

--- math-font-fallback ---
// Test font fallback.
$ よ and 🏳️‍🌈 $

--- math-text-color ---
// Test text properties.
$text(#red, "time"^2) + sqrt("place")$

--- math-equation-font ---
// Test different font.
#show math.equation: set text(font: "Fira Math")
$ v := vec(1 + 2, 2 - 4, sqrt(3), arrow(x)) + 1 $

--- math-symbol-show-rule ---
// Test using rules for symbols
#show sym.tack: it => $#h(1em) it #h(1em)$
$ a tack b $
