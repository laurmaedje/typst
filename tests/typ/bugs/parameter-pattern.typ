// Test that underscore works in parameter patterns.

--- issue-1029-parameter-destructuring ---
#test((1, 2, 3).zip((1, 2, 3)).map(((_, x)) => x), (1, 2, 3))
