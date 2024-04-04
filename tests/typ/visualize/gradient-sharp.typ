// Test sharp gradients.

--- gradient-linear-sharp ---
#square(
  size: 100pt,
  fill: gradient.linear(..color.map.rainbow, space: color.hsl).sharp(10),
)
#square(
  size: 100pt,
  fill: gradient.radial(..color.map.rainbow, space: color.hsl).sharp(10),
)
#square(
  size: 100pt,
  fill: gradient.conic(..color.map.rainbow, space: color.hsl).sharp(10),
)

--- gradient-linear-sharp-and-smooth ---
#square(
  size: 100pt,
  fill: gradient.linear(..color.map.rainbow, space: color.hsl).sharp(10, smoothness: 40%),
)
#square(
  size: 100pt,
  fill: gradient.radial(..color.map.rainbow, space: color.hsl).sharp(10, smoothness: 40%),
)
#square(
  size: 100pt,
  fill: gradient.conic(..color.map.rainbow, space: color.hsl).sharp(10, smoothness: 40%),
)
