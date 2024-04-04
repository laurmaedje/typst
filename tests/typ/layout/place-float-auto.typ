// Test floating placement.

--- place-float ---
#set page(height: 140pt)
#set place(clearance: 5pt)
#lorem(6)
#place(auto, float: true, rect[A])
#place(auto, float: true, rect[B])
#place(auto, float: true, rect[C])
#place(auto, float: true, rect[D])

--- place-float-missing ---
// Error: 2-20 automatic positioning is only available for floating placement
// Hint: 2-20 you can enable floating placement with `place(float: true, ..)`
#place(auto)[Hello]

--- place-float-center-horizon ---
// Error: 2-45 floating placement must be `auto`, `top`, or `bottom`
#place(center + horizon, float: true)[Hello]

--- place-float-horizon ---
// Error: 2-36 floating placement must be `auto`, `top`, or `bottom`
#place(horizon, float: true)[Hello]

--- place-float-default ---
// Error: 2-27 floating placement must be `auto`, `top`, or `bottom`
#place(float: true)[Hello]

--- place-float-right ---
// Error: 2-34 floating placement must be `auto`, `top`, or `bottom`
#place(right, float: true)[Hello]
