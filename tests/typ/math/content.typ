// Test arbitrary content in math.

--- math-nested-normal-layout ---
// Test images and font fallback.
#let monkey = move(dy: 0.2em, image("/assets/images/monkey.svg", height: 1em))
$ sum_(i=#emoji.apple)^#emoji.apple.red i + monkey/2 $

--- math-table ---
// Test tables.
$ x := #table(columns: 2)[x][y]/mat(1, 2, 3)
     = #table[A][B][C] $

--- math-equation-auto-wrapping ---
// Test non-equation math directly in content.
#math.attach($a$, t: [b])

--- math-font-switch ---
// Test font switch.
#let here = text.with(font: "Noto Sans")
$#here[f] := #here[Hi there]$.

--- math-box-without-baseline ---
// Test boxes without a baseline act as if the baseline is at the base
#{
  box(stroke: 0.2pt, $a #box(stroke: 0.2pt, $a$)$)
  h(12pt)
  box(stroke: 0.2pt, $a #box(stroke: 0.2pt, $g$)$)
  h(12pt)
  box(stroke: 0.2pt, $g #box(stroke: 0.2pt, $g$)$)
}

--- math-box-with-baseline ---
// Test boxes with a baseline are respected
#box(stroke: 0.2pt, $a #box(baseline:0.5em, stroke: 0.2pt, $a$)$)
