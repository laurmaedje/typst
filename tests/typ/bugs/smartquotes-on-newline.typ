// Test that smart quotes are inferred correctly across newlines.

--- issue-1540-smartquotes-across-newlines ---
"test"#linebreak()"test"

"test"\
"test"
