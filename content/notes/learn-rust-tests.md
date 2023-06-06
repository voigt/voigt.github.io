---
title: "#learn-rust: tests"
date: 2023-05-23
author: Christoph Voigt
showAuthor: false
published: true
tags:
  - rust
  - learn-rust
  - notes
---

Syntax to write tests in Rust:

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod add_function_tests {
    use super::*;

    #[test]
    fn add_works() {
        assert_eq!(add(1, 2), 3);
        assert_eq!(add(10, 12), 22);
        assert_eq!(add(5, -2), 3);
    }

    #[test]
    #[should_panic]
    fn add_fails() {
        assert_eq!(add(2, 2), 7);
    }

    #[test]
    #[ignore]
    fn add_negatives() {
        assert_eq!(add(-2, -2), -4)
    }
}
```

The `cfg` attribute controls conditional compilation and will only compile the thing it's attached to if the predicate is `true`. The `test` compilation flag is issued automatically by Cargo whenever we execute the command `$ cargo test`, so it will always be true when we run our tests.
The `use super::*;` declaration is necessary for the code inside the `add_function_tests` module to access the `add` in the outer module.

