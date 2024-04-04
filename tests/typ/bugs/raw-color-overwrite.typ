// Test that the color of a raw block is not overwritten

--- issue-2259-raw-color-overwrite ---

#show raw: set text(fill: blue)

`Hello, World!`

```rs
fn main() {
    println!("Hello, World!");
}
```
