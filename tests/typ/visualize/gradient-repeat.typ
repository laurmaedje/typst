// Test repeated gradients.

--- gradient-linear-repeat-and-mirror-1 ---
#rect(
  height: 40pt,
  width: 100%,
  fill: gradient.linear(..color.map.inferno).repeat(2, mirror: true)
)

--- gradient-linear-repeat-and-mirror-2 ---
#rect(
  height: 40pt,
  width: 100%,
  fill: gradient.linear(..color.map.rainbow).repeat(2, mirror: true),
)

--- gradient-linear-repeat-and-mirror-3 ---
#rect(
  height: 40pt,
  width: 100%,
  fill: gradient.linear(..color.map.rainbow).repeat(5, mirror: true)
)

--- gradient-linear-sharp-and-repeat ---
#rect(
  height: 40pt,
  width: 100%,
  fill: gradient.linear(..color.map.rainbow).sharp(10).repeat(5, mirror: false)
)

--- gradient-linear-sharp-repeat-and-mirror ---
#rect(
  height: 40pt,
  width: 100%,
  fill: gradient.linear(..color.map.rainbow).sharp(10).repeat(5, mirror: true)
)
