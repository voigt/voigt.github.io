---
title: "#learn-rust What is the question mark (?) operator?"
date: 2023-06-25
author: Christoph Voigt
showAuthor: false
published: 2023-06-25
tags:
  - rust
  - learn-rust
  - notes
---

The question mark operator provides an alternative and concise approach to handle errors in functions that return the `Result` or `Option` type. By using the `?` operator, we can reduce the amount of code needed to handle returning `Err` or `None` from `Result<T, Err>` or `Option<T>` types.

Here's a quick example:

Instead of using a `match` statement to check the success of a function `do_something()` that returns a `Result<T, Err>` type, we can achieve the same result with the following shorter syntax:

```rust
let a = match do_something() {
  Ok(a) => a,
  Err(e) => return Err(e),
}
```

While this is quite expressive, it can be shortened to:

```rust
let a = do_something()?;
```

This operator unwraps valid values and propagates erroneous values to the calling function.

Now, the question arises: should we always use `?`? Well, as is often the case, it depends.

The `?` operator simplifies code and enhances readability by reducing boilerplate. However, it can only be used in functions that return `Result` or `Option` types. Therefore, changing the return type of a function is necessary to utilize this operator, which may not always be feasible or desirable. Furthermore, the `?` operator is not available in `main()` functions.

Additionally, while the `?` operator is thorough and concise, it lacks the ability to provide custom behavior. There might be situations where you prefer to:

- Invoke custom behavior, such as error recovery
- Create a custom error
- Even panic

In such cases, the match statement is still required to fulfill these specific needs.

Let's consider another example that demonstrates error propagation across multiple function calls:

```rust
use std::io;

fn get_user_age() -> Result<u8, io::Error> {
    // Simulating an error condition
    Err(io::Error::new(std::io::ErrorKind::Other, "could not get age!"))
}

fn get_user_name() -> Result<String, io::Error> {
    // Simulating a successful operation
    Ok("Alice".to_string())
}

fn process_user_data() -> Result<(), io::Error> {
    let _ = get_user_name()?;
    let _ = get_user_age()?;
  
    // Processing user data...
    Ok(())
}

fn main() {
    match process_user_data() {
        Ok(()) => println!("User data processed successfully"),
        Err(e) => println!("Error: {}", e),
    }
}
```

Since `get_user_age()` encounters an error, the program will terminate and display the following message:

```bash
Error: could not get age!
```

Well, this is all I know about the `?` operator. I hope you found this useful.

**References**

- [Rust by Example - 19.5.1 Result - ?](https://doc.rust-lang.org/rust-by-example/std/result/question_mark.html)
- [Recoverable Errors with `Result`](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html#recoverable-errors-with-result)
