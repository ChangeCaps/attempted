# Attempted

[![Crates.io](https://img.shields.io/crates/v/attempted)](https://crates.io/crates/attempted)
[![Documentation](https://img.shields.io/docsrs/attempted)](https://docs.rs/attempted/latest)

## Examples

```rust
fn positive(x: i32) -> Option<i32> {
    if x > 0 {
        Some(x)
    } else {
        None
    }
}

#[attempt]
fn test() {
    // try something
    let x = positive(13)?;
    
    // do something with x
    println!("{} is positive", x);
}
```
