---
title: "unable to find zig installation directory: FileNotFound"
date: 2022-06-02
author: Christoph Voigt
showAuthor: false
tags:
  - zig
  - wasm
  - notes
---

If zig refused to build because it is "unable to find zig installation directory", you need to ensure the lib directory resides at the same relative path as the zig binary.

I played around and wanted to create a container image to streamline my Zig build experience. Doing so, I used the [official tarball](https://ziglang.org/download/index.json) and copied over the `zig` binary, put it into `/usr/local/bin`, and assumed this was going to work. `zig version` even proved me correct:

```
$ zig version
0.10.0-dev.2431+0e6285c8f
```

However, once I wanted to build, I got this error message:

```bash
error: unable to find zig installation directory '/usr/local/bin/zig': FileNotFound
```

As usual, Zig error messages aren't that helpful (yet). Only coincidentally, I found an [old dump of zigs freenode IRC channel](https://freenode.irclog.whitequark.org/zig/2021-02-16#29154913;) pointing me in the right direction:

> [21:25](https://freenode.irclog.whitequark.org/zig/2021-02-16#29154936) <ifreund> the `zig` binary needs to stay at the same relative path to the `lib` dir

So the solution is not only to copy the binary but also the lib directory from the tarball.

Hopefully, this will be picked up by the search engines, so next time someone encounters the issue, it's a little bit less of a riddle.