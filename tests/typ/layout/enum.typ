// Test enumerations.

--- enum-function-call ---
#enum[Embrace][Extend][Extinguish]

--- enum-number-override-nested ---
0. Before first!
1. First.
   2. Indented

+ Second

--- enum-built-in-loop ---
// Test automatic numbering in summed content.
#for i in range(5) {
   [+ #numbering("I", 1 + i)]
}

--- list-mix ---
// Mix of different lists
- Bullet List
+ Numbered List
/ Term: List

--- enum-syntax-at-start ---
// In the line.
1.2 \
This is 0. \
See 0.3. \

--- enum-syntax-edge-cases ---
// Edge cases.
+
Empty \
+Nope \
a + 0.

--- enum-number-override ---
// Test item number overriding.
1. first
+ second
5. fifth

#enum(
   enum.item(1)[First],
   [Second],
   enum.item(5)[Fifth]
)
