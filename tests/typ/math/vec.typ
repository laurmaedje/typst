// Test vectors.

--- math-vec-wide ---
// Test wide cell.
$ v = vec(1, 2+3, 4) $

--- math-vec-delim-set ---
// Test alternative delimiter.
#set math.vec(delim: "[")
$ vec(1, 2) $

--- math-vec-delim-invalid ---
// Error: 22-25 expected "(", "[", "{", "|", "||", or none
#set math.vec(delim: "%")
