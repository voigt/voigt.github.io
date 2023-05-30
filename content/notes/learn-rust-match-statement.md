---
title: "#learn-rust: retrieving optional values"
date: 2023-05-25
author: Christoph Voigt
showAuthor: false
published: true
tags:
  - rust
  - learn-rust
  - notes
---

match statement #learn-rust

```rust
let fruits = vec!["banana", "apple", "coconut", "orange", "strawberry"];
for &index in [0, 2, 99].iter() {
    match fruits.get(index) {
        // note the "&" to get the Option<&&str>
        Some(&"coconut") => println!("Coconuts are awesome!!!"),
        Some(fruit_name) => println!("It's a delicious {}!", fruit_name),
        None => println!("There is no fruit! :("),
    }
}
```

Whenever you use the _match_ expression, keep the following rules in mind:

- `match` arms are evaluated from top to bottom. Specific cases must be defined earlier than generic cases or they'll never be matched and evaluated.
- `match` arms must cover every possible value that the input type could have. You'll get a compiler error if you try to match against a non-exhaustive pattern list.

---

## Use pattern matching for convenience

If we expect a specific `Some` value in an `Option`, we could use match (and drop anything else):

```rust
let a_number: Option = Some(7);
match a_number {
    Some(7) => println!("That's my lucky number!"),
    _ => {},
}
```

Or use a shorter, equivalent version:

```rust
let a_number: Option = Some(7);
if let Some(7) = a_number {
    println!("That's my lucky number!");
}
```

---

## Use `unwrap` and `expect` carefully

`unwrap` and `expect` can be used, but will panic if `Option` is `None`

```rust
let gift = Some("candy");
assert_eq!(gift.unwrap(), "candy");

let empty_gift: Option<&str> = None;
assert_eq!(empty_gift.unwrap(), "candy"); // This will panic!
```

This will panic like this:

```bash
thread 'main' panicked at 'called `Option::unwrap()` on a `None` value', src/main.rs:6:27
```

With `expect` we can set a custom panic message:

```rust
let a = Some("value");
assert_eq!(a.expect("fruits are healthy"), "value");

let b: Option<&str> = None;
b.expect("fruits are healthy"); // panics with `fruits are healthy`
```

```bash
thread 'main' panicked at 'fruits are healthy', src/main.rs:6:7
```

As both, `unwrap` and `expect` can panic it is **recommended to avoid them** and use either of following options instead:

1. Use pattern matching and handle the `None` case explicitly.
2. Call similar non-panicking methods, such as `unwrap_or`, which returns a default value if the variant is `None` or the inner value if the variant is `Some(value)`.  
```rust
assert_eq!(Some("dog").unwrap_or("cat"), "dog");
assert_eq!(None.unwrap_or("cat"), "cat");
```