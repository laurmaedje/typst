// Tests whether hue rotation works correctly.

--- gradient-linear-oklab ---
// Test in Oklab space for reference.
#set page(
  width: 100pt,
  height: 30pt,
  fill: gradient.linear(red, purple, space: oklab)
)

--- gradient-linear-oklch ---
// Test in OkLCH space.
#set page(
  width: 100pt,
  height: 30pt,
  fill: gradient.linear(red, purple, space: oklch)
)

--- gradient-linear-hsv ---
// Test in HSV space.
#set page(
  width: 100pt,
  height: 30pt,
  fill: gradient.linear(red, purple, space: color.hsv)
)

--- gradient-linear-hsl ---
// Test in HSL space.
#set page(
  width: 100pt,
  height: 30pt,
  fill: gradient.linear(red, purple, space: color.hsl)
)

--- gradient-conic-oklab ---
// Test in Oklab space for reference.
#set page(
  width: 100pt,
  height: 100pt,
  fill: gradient.conic(red, purple, space: oklab)
)

--- gradient-conic-oklch ---
// Test in OkLCH space.
#set page(
  width: 100pt,
  height: 100pt,
  fill: gradient.conic(red, purple, space: oklch)
)

--- gradient-conic-hsv ---
// Test in HSV space.
#set page(
  width: 100pt,
  height: 100pt,
  fill: gradient.conic(red, purple, space: color.hsv)
)

--- gradient-conic-hsl ---
// Test in HSL space.
#set page(
  width: 100pt,
  height: 100pt,
  fill: gradient.conic(red, purple, space: color.hsl)
)
