# supermemo2

This crate implements the core components of the supermemo2 spaced repetition algorithm.

# Examples

```rust
use supermemo2::Item;

pub fn main() {
    let item = Item::default();
    let interval = item
        .review(4)
        .unwrap()
        .review(3)
        .unwrap()
        .review(5)
        .unwrap()
        .interval();

    assert_eq!(interval, 15);
}
```
