---
title: "#learn-rust Rust enums"
date: 2023-06-18
author: Christoph Voigt
showAuthor: false
published: true
tags:
  - rust
  - learn-rust
  - notes
---

Defining an enum is not very spectacular

```rust
enum Color {
  Yellow,
  Red,
  Green,
  Blue,
}
```

Let's pattern-match against the enum, forgetting to match one of the variants. The compiler will warn us about this.

```rust
fn print_color(color: Color) {
  match color {
    Color::Yellow => println!("Yellow"),
    Color::Red => println!("Red"),
    Color::Green => println!("Green"),
    // Color::Blue => println!("Blue"),
  }
}
```

Attaching methods to enums is also possible 🤯

```rust
impl Color {
  fn green_part(&self) -> bool {
    match self {
      Color::Yellow => true,
      Color::Blue => true,
      _ => false,
    }
  }

  fn is_green(&self) -> bool {
    // pattern matching self on the if-statement
    if let Color::Green = self {
      return true;
    }
    return false
  }
}
```
