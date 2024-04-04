// Test that gradients are applied correctly on equations.

--- gradient-math-cancel ---
// Test on cancel
#show math.equation: set text(fill: gradient.linear(..color.map.rainbow))
#show math.equation: box

$ a dot cancel(5) = cancel(25) 5 x + cancel(5) 1 $

--- gradient-math-frac ---
// Test on frac
#show math.equation: set text(fill: gradient.linear(..color.map.rainbow))
#show math.equation: box

$ nabla dot bold(E) = frac(rho, epsilon_0) $

--- gradient-math-root ---
// Test on root
#show math.equation: set text(fill: gradient.linear(..color.map.rainbow))
#show math.equation: box

$ x_"1,2" = frac(-b +- sqrt(b^2 - 4 a c), 2 a) $

--- gradient-math-mat ---
// Test on matrix
#show math.equation: set text(fill: gradient.linear(..color.map.rainbow))
#show math.equation: box

$ A = mat(
  1, 2, 3;
  4, 5, 6;
  7, 8, 9
) $

--- gradient-math-underover ---
// Test on underover
#show math.equation: set text(fill: gradient.linear(..color.map.rainbow))
#show math.equation: box

$ underline(X^2) $
$ overline("hello, world!") $

--- gradient-math-dir ---
// Test a different direction
#show math.equation: set text(fill: gradient.linear(..color.map.rainbow, dir: ttb))
#show math.equation: box

$ A = mat(
  1, 2, 3;
  4, 5, 6;
  7, 8, 9
) $

$ x_"1,2" = frac(-b +- sqrt(b^2 - 4 a c), 2 a) $

--- gradient-math-misc ---
// Test miscellaneous
#show math.equation: set text(fill: gradient.linear(..color.map.rainbow))
#show math.equation: box

$ hat(x) = bar x bar = vec(x, y, z) = tilde(x) = dot(x) $
$ x prime = vec(1, 2, delim: "[") $
$ sum_(i in NN) 1 + i $
$ attach(
  Pi, t: alpha, b: beta,
  tl: 1, tr: 2+3, bl: 4+5, br: 6,
) $

--- gradient-math-radial ---
// Test radial gradient
#show math.equation: set text(fill: gradient.radial(..color.map.rainbow, center: (30%, 30%)))
#show math.equation: box

$ A = mat(
  1, 2, 3;
  4, 5, 6;
  7, 8, 9
) $

--- gradient-math-conic ---
// Test conic gradient
#show math.equation: set text(fill: gradient.conic(red, blue, angle: 45deg))
#show math.equation: box

$ A = mat(
  1, 2, 3;
  4, 5, 6;
  7, 8, 9
) $
