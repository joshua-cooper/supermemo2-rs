# supermemo2

This crate implements the core components of the supermemo2 spaced repetition algorithm.

# Examples

```rust
use supermemo2::{Item, Quality};
let item = Item::new();
let quality = Quality::from(5).unwrap();
let interval = item
    .answer(&quality)
    .answer(&quality)
    .answer(&quality)
    .interval();

assert_eq!(interval, 17);
```
