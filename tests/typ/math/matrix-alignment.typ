// Test matrix alignment math.

--- math-vec-align-explicit-alternating ---
// Test alternating alignment in a vector.
$ vec(
  "a" & "a a a" & "a a",
  "a a" & "a a" & "a",
  "a a a" & "a" & "a a a",
) $

--- math-mat-align-explicit--alternating ---
// Test alternating explicit alignment in a matrix.
$ mat(
  "a" & "a a a" & "a a";
  "a a" & "a a" & "a";
  "a a a" & "a" & "a a a";
) $

--- math-mat-align-implicit ---
// Test alignment in a matrix.
$ mat(
  "a", "a a a", "a a";
  "a a", "a a", "a";
  "a a a", "a", "a a a";
) $

--- math-mat-align-explicit-left ---
// Test explicit left alignment in a matrix.
$ mat(
  &"a", &"a a a", &"a a";
  &"a a", &"a a", &"a";
  &"a a a", &"a", &"a a a";
) $

--- math-mat-align-explicit-right ---
// Test explicit right alignment in a matrix.
$ mat(
  "a"&, "a a a"&, "a a"&;
  "a a"&, "a a"&, "a"&;
  "a a a"&, "a"&, "a a a"&;
) $

--- math-mat-align-complex ---
// Test #460 equations.
#let stop = {
  math.class("punctuation",$.$)
}
$ mat(&a+b,c;&d, e) $
$ mat(&a+b&,c;&d&, e) $
$ mat(&&&a+b,c;&&&d, e) $
$ mat(stop &a+b&stop,c;...stop stop&d&...stop stop, e) $

--- math-mat-align-signed-numbers ---
// Test #454 equations.
$ mat(-1, 1, 1; 1, -1, 1; 1, 1, -1) $
$ mat(-1&, 1&, 1&; 1&, -1&, 1&; 1&, 1&, -1&) $
$ mat(-1&, 1&, 1&; 1, -1, 1; 1, 1, -1) $
$ mat(&-1, &1, &1; 1, -1, 1; 1, 1, -1) $
