// Test list marker configuration.

--- list-marker-dash ---
// Test en-dash.
#set list(marker: [--])
- A
- B

--- list-marker-cycle ---
// Test that items are cycled.
#set list(marker: ([--], [•]))
- A
  - B
    - C

--- list-marker-closure ---
// Test function.
#set list(marker: n => if n == 1 [--] else [•])
- A
- B
  - C
  - D
    - E
- F

--- list-marker-bare-hyphen ---
// Test that bare hyphen doesn't lead to cycles and crashes.
#set list(marker: [-])
- Bare hyphen is
- a bad marker

--- list-marker-array-empty ---
// Error: 19-21 array must contain at least one marker
#set list(marker: ())
