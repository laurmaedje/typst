// Test bullet lists.

--- list-basic ---
_Shopping list_
#list[Apples][Potatoes][Juice]

--- list-nested ---
- First level.

  - Second level.
    There are multiple paragraphs.

    - Third level.

    Still the same bullet point.

  - Still level 2.

- At the top.

--- list-content-block ---
- Level 1
  - Level #[
2 through content block
]

--- list-top-level-indent ---
  - Top-level indent
- is fine.

--- list-indent-specifics ---
 - A
     - B
   - C
- D

--- list-tabs ---
// This works because tabs are used consistently.
	- A with 1 tab
		- B with 2 tabs

--- list-mixed-tabs-and-spaces ---
// This doesn't work because of mixed tabs and spaces.
  - A with 2 spaces
		- B with 2 tabs

--- list-syntax-edge-cases ---
// Edge cases.
-
Not in list
-Nope

--- list-marker-align-unaffected ---
// Alignment shouldn't affect marker
#set align(horizon)

- ABCDEF\ GHIJKL\ MNOPQR
