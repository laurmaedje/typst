// Test emph and strong.

--- emph-syntax ---
// Basic.
_Emphasized and *strong* words!_

// Inside of a word it's a normal underscore or star.
hello_world Nutzer*innen

// CJK characters will not need spaces.
中文一般使用*粗体*或者_楷体_来表示强调。

日本語では、*太字*や_斜体_を使って強調します。

中文中混有*Strong*和_Empasis_。

// Can contain paragraph in nested content block.
_Still #[

] emphasized._

--- emph-and-strong-call-in-word ---
// Inside of words can still use the functions.
P#strong[art]ly em#emph[phas]ized.

--- strong-delta ---
// Adjusting the delta that strong applies on the weight.
Normal

#set strong(delta: 300)
*Bold*

#set strong(delta: 150)
*Medium* and *#[*Bold*]*

--- emph-unclosed ---
// Error: 6-7 unclosed delimiter
#box[_Scoped] to body.

--- emph-ends-at-parbreak ---
// Ends at paragraph break.
// Error: 1-2 unclosed delimiter
_Hello

World

--- emph-strong-unclosed-nested ---
// Error: 11-12 unclosed delimiter
// Error: 3-4 unclosed delimiter
#[_Cannot *be interleaved]
