---
author: Christoph Voigt
title: Zig File Open in WASI via preopen
date: 2022-04-28
description: Zig File Open in WASI via preopen
tags:
  - zig
  - wasm
---

WebAssembly programs are running in a secure runtime. Secure in this context also means almost no information sharing between the host and your program. This can become impractical pretty soon when it comes to standard procedures like reading files, network, or other host i/o. Luckily, there is an extension spec to the existing WebAssembly spec called WASI - WebAssembly System Interface.

### What is WASI?

WASI was designed by the Wasmtime project as an extension to WebAssembly to propose an API that is standardized, engine-independent, and system-oriented. The initial Focus of WASI Core is File Access and Networking (with others to come!). While the spec for File Access is already done and implemented in many languages, network still needs to be awaited.

### Sharing a file from Host to Sandbox

To open Files that exist on the Host, WASI follows the preopen pattern. Modules may be granted capabilities for directories on launch. The WASI library then maintains a mapping from their filesystem path to the file descriptor indices representing the associated capabilities. When a program calls [open](http://pubs.opengroup.org/onlinepubs/009695399/functions/open.html), they look up the file name in the map and automatically supply the appropriate directory capability.
This capability-based handling of files is a pretty interesting topic. If you want to read more on how it is implemented, I recommend reading about [libpreopen](https://github.com/musec/libpreopen) or the [WASI Overview](https://github.com/bytecodealliance/wasmtime/blob/main/docs/WASI-overview.md#capability-oriented) page.

### Example

Zig has this preopen procedure already implemented as a part of the standard library. Feel free to look up the [code](https://github.com/ziglang/zig/blob/master/lib/std/fs/wasi.zig#L78); it isn't too intimidating.
Before we start, we need to initialize an allocator that serves as a dedicated space to keep our preopens in memory:

```zig
    var general_purpose_allocator = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = general_purpose_allocator.allocator();
```

In this case, I use the GeneralPurposeAllocator, which should be good enough for this example. However, potentially this is an area for improvement, depending on your scenario.
Next, we are initializing the list of preopens and storing them in a `preopens` variable.

```zig
    var preopens = std.fs.wasi.PreopenList.init(allocator);
    defer preopens.deinit();
    try preopens.populate();
```

Note that you can hand over multiple files or directories at launch. That's why PreopenList is an ArrayList.
Once we have our list of preopens, we can iterate over and print them:

```zig
    for (preopens.asSlice()) |preopen, i| {
        std.debug.print("{}: {}\n", .{ i, preopen });
    }
```

Please note that this list has many more handy functions, such as [`find`](https://github.com/ziglang/zig/blob/master/lib/std/fs/wasi.zig#L254) or [`findByFd`](https://github.com/ziglang/zig/blob/master/lib/std/fs/wasi.zig#L243) to search for a specific preopen.
Though, so far, we haven't done anything with our files. We just ensured that they were available within our sandbox. For actual access, we need to initialize the available Preopen list on WASI and set our path (CWD).

```zig
    try os.initPreopensWasi(allocator, ".");
```

This must be called before using any relative or absolute paths with `std.os` functions. In my specific case, I'm working with the current directory (`"."`), which is the one I'm going to hand over on the program launch. In your case, you might want to have this a little more variable.
For notation, please note the [docs](https://github.com/ziglang/zig/blob/master/lib/std/os.zig#L1445-L1450):

```
/// The current working directory is initialized to `cwd_root`, and `cwd_root`
/// is inserted as a prefix for any Preopens whose dir begins with "."
///   For example:
///      "./foo/bar" - canonicalizes to -> "{cwd_root}/foo/bar"
///      "foo/bar"   - canonicalizes to -> "/foo/bar"
///      "/foo/bar"  - canonicalizes to -> "/foo/bar"
///
/// `cwd_root` must be an absolute path. For initialization behavior similar to
/// wasi-libc, use "/" as the `cwd_root`
///
/// `cwd_root` must be an absolute path. For initialization behavior similar to
/// wasi-libc, use "/" as the `cwd_root`
```

Afterwards, we can do file actions in the well-known Zig-style. Therefore we open our file and defer closing it. Now we can print its contents or - like in my example - do a stat on the file.

```zig
    var file = try std.fs.cwd().openFile(args[1], .{});
    defer file.close();
    var stat = try file.stat();
    std.debug.print("{}\n", .{ stat });
```

This is the entire Source Code:

```zig
const std = @import("std");

pub fn main() !void {
    var general_purpose_allocator = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = general_purpose_allocator.allocator();

    var preopens = std.fs.wasi.PreopenList.init(allocator);
    defer preopens.deinit();
    try preopens.populate();

    for (preopens.asSlice()) |preopen, i| {
        std.debug.print("{}: {}\n", .{ i, preopen });
    }

    try std.os.initPreopensWasi(allocator, ".");

    var file = try std.fs.cwd().openFile("test.txt", .{});
    defer file.close();
    var stat = try file.stat();
    std.debug.print("{}\n", .{ stat });
}
```

Now we need to build our program for Wasi:

```bash
$ zig build-exe src/main.zig -target wasm32-wasi
```

And execute it:

```bash
$ wasmtime --dir=. --dir=/Users/cvoigt main.wasm test.txt
```

Obviously, with `--dir=.` I'm declaring the current directory. Consequently, the file I'm opening should exist in that same directory. If you want to hand over a specific directory, you could also do something like this: `--dir=/Users/cvoigt`, which in my case, would hand over my home directory.
So the following would be valid:

```bash
$ wasmtime --dir=. --dir=/Users/cvoigt main.wasm test.txt
```

While WASI preopens are [part of the documentation](https://ziglang.org/documentation/0.9.1/#WASI), I had to do some research to actually understand what it is and how to work with preopened files. This is most likely because I'm still pretty new to Zig. However, there isn't too much material on that topic available, so hopefully, this is a helpful contribution to anyone out there.
