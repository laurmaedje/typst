// Test list attaching.

--- list-attached ---
// Test basic attached list.
Attached to:
- the bottom
- of the paragraph

Next paragraph.

--- list-attached-above-spacing ---
// Test that attached list isn't affected by block spacing.
#show list: set block(above: 100pt)
Hello
- A
World
- B

--- list-non-attached-followed-by-attached ---
// Test non-attached list followed by attached list,
// separated by only word.
Hello

- A

World
- B

--- list-tight-non-attached-tight ---
// Test non-attached tight list.
#set block(spacing: 15pt)
Hello
- A
World

- B
- C

More.

--- list-wide-cannot-attach ---
// Test that wide lists cannot be ...
#set block(spacing: 15pt)
Hello
- A

- B
World

--- list-wide-really-cannot-attach ---
// ... even if forced to.
Hello
#list(tight: false)[A][B]
World
