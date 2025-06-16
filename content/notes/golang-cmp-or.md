---
title: Handling Environment Variables Gracefully in Go
date: 2025-05-23
published: 2025-05-23
author: Christoph Voigt
showAuthor: false
draft: false
tags:
  - notes
  - golang
---
When writing Go applications, it's common to configure runtime behavior using environment variables. But what happens when an environment variable isn’t set? Falling back to a default value is a typical pattern. The `cmp` package’s `Or` function, [introduced in Go 1.22](https://go.dev/doc/go1.22#minor_library_changes) provides a clean way to express this.

### The Problem

Admittedly, the word "problem" is perhaps a bit of an exaggeration. Anyways, a common pattern to retrieve a value from the environment with a fallback looks like this:

```go
package main

import (
	"fmt"
	"os"
)

func main() {
	port := os.Getenv("APP_PORT")
	if port == "" {
		port = "8080"
	}
	fmt.Println(port)
}
```

This works, but it's a bit verbose. If you're doing this repeatedly across many config values, the boilerplate adds up.

### A Cleaner Alternative: `cmp.Or`

The `cmp.Or` function provides a neater, more expressive way to write this fallback logic:

```go
package main

import (
	"cmp"
	"fmt"
	"os"
)

func main() {
	port := cmp.Or(os.Getenv("APP_PORT"), "8080")
	fmt.Println(port)
}
```

`cmp.Or` receives an arbitrary amount of arguments and returns the first non-zero (i.e. non-empty for strings) value it encounters. If `APP_PORT` is set, it uses that. If not, it falls back to `"8080"`.

This means it could be used for checking multiple sources:

```go
	port := cmp.Or(os.Getenv("APP_PORT"), os.Getenv("PORT"), "8080")
```

This pattern is especially useful when initializing configuration from multiple potential sources, e.g. environment → config file → hardcoded default.

Apparently, people use `cmp.Or` for more than configuration. [Multi-field sorting](https://brandur.org/fragments/cmp-or-multi-field) is also an interesting application.