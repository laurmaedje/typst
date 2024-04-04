// Test multiline math.

--- math-align-basic ---
// Test basic alignment.
$ x &= x + y \
    &= x + 2z \
    &= sum x dot 2z $

--- math-align-wider-first-column ---
// Test text before first alignment point.
$ x + 1 &= a^2 + b^2 \
      y &= a + b^2 \
      z &= alpha dot beta $

--- math-align-aligned-in-source ---
// Test space between inner alignment points.
$ a + b &= 2 + 3 &= 5 \
      b &= c     &= 3 $

--- math-align-cases ---
// Test in case distinction.
$ f := cases(
  1 + 2 &"iff" &x,
  3     &"if"  &y,
) $

--- math-align-lines-mixed ---
// Test mixing lines with and some without alignment points.
$ "abc" &= c \
   &= d + 1 \
   = x $

--- math-attach-subscript-multiline ---
// Test multiline subscript.
$ sum_(n in NN \ n <= 5) n = (5(5+1))/2 = 15 $

--- math-multiline-no-trailing-linebreak ---
// Test no trailing line break.
$
"abc" &= c
$
No trailing line break.

--- math-multiline-trailing-linebreak ---
// Test single trailing line break.
$
"abc" &= c \
$
One trailing line break.

--- math-multiline-multiple-trailing-linebreaks ---
// Test multiple trailing line breaks.
$
"abc" &= c \ \ \
$
Multiple trailing line breaks.
