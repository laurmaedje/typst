// Test conic gradients

--- gradient-conic ---
#square(
  size: 50pt,
  fill: gradient.conic(..color.map.rainbow, space: color.hsv),
)

--- gradient-conic-center-shifted-1 ---
#square(
  size: 50pt,
  fill: gradient.conic(..color.map.rainbow, space: color.hsv, center: (10%, 10%)),
)

--- gradient-conic-center-shifted-2 ---
#square(
  size: 50pt,
  fill: gradient.conic(..color.map.rainbow, space: color.hsv, center: (90%, 90%)),
)

--- gradient-conic-angled ---
#square(
  size: 50pt,
  fill: gradient.conic(..color.map.rainbow, space: color.hsv, angle: 90deg),
)
