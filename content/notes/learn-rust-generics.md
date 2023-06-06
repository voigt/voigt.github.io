---
title: "#learn-rust: generics"
date: 2023-05-24
author: Christoph Voigt
showAuthor: false
published: true
tags:
  - rust
  - learn-rust
  - notes
---

Syntax of a generic `struct`:

```
struct Point {
    x: T,
    y: T,
}

fn main() {
    let boolean = Point { x: true, y: false };
    let integer = Point { x: 1, y: 9 };
    let float = Point { x: 1.7, y: 4.3 };
    let string_slice = Point { x: "high", y: "low" };
}
  ```

Note, that multiple different types are possible:

```
struct Point {
    x: T,
    y: U,
}

fn main() {
    let integer_and_boolean = Point { x: 5, y: false };
    let float_and_string = Point { x: 1.0, y: "hey" };
    let integer_and_float = Point { x: 5, y: 4.0 };
    let both_integer = Point { x: 10, y: 30 };
    let both_boolean = Point { x: true, y: true };
}
```
