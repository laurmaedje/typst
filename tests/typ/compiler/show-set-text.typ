// Text show-set rules are weird.

--- show-set-text-order-adjacent-1 ---
#show "He": set text(red)
#show "ya": set text(blue)
Heya

--- show-set-text-order-contained-1 ---
#show "Heya": set text(red)
#show   "ya": set text(blue)
Heya

--- show-set-text-order-contained-3 ---
#show "He": set text(red)
#show "Heya": set text(blue)
Heya

--- show-set-text-order-overlapping-1 ---
#show "Heya": set text(red)
#show   "yaho": set text(blue)
Heyaho

--- show-set-text-order-adjacent-2 ---
#show "He": set text(red)
#show "ya": set text(weight: "bold")
Heya

--- show-set-text-order-contained-2 ---
#show "Heya": set text(red)
#show   "ya": set text(weight: "bold")
Heya

--- show-set-text-order-contained-4 ---
#show "He": set text(red)
#show "Heya": set text(weight: "bold")
Heya

--- show-set-text-order-overlapping-2 ---
#show "Heya": set text(red)
#show   "yaho": set text(weight: "bold")
Heyaho
