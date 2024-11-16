---
title: "Justfile with polyglot support"
date: 2023-06-22
published: 2023-06-22
author: Christoph Voigt
showAuthor: false
tags:
  - notes
---

I just discovered that the [just](https://just.systems/) command runner supports [running commands in multiple languages](https://just.systems/man/en/chapter_41.html)! As I will probably forget this, I will write it down here.

```bash
py:
  #!/usr/bin/env python3
  name = "Python"
  print(f'Greetings from {name}!')

js:
  #!/usr/bin/env node
  const name = "JavaScript";
  console.log(`Greetings from ${name}!`)

go:
  #!/Users/c.voigt/go/bin/gorun
  package main

  import "fmt"

  func main() {
      var name = "Go"
      fmt.Printf("Greetings from %s!\n", name)
  }

sh:
  #!/usr/bin/env sh
  name="Shell"
  echo "Greetings from ${name}!"

rhai:
  #!/Users/c.voigt/.cargo/bin/rhai-run

  let answer = "Rhai";
  print(`Greetings from ${answer}`);

```

To run go code I installed [gorun](https://github.com/erning/gorun). Admittedly this is a bit hacky, but certainly does the job.

To use Rhai install it with cargo: `cargo install rhai` and check `which rhai-run` to get the path to the executable.
