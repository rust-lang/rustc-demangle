# rustc-demangle

Symbol demangling for Rust

[![Build Status](https://travis-ci.org/alexcrichton/rustc-demangle.svg?branch=master)](https://travis-ci.org/alexcrichton/rustc-demangle)

[Documentation](http://alexcrichton.com/rustc-demangle)

# Usage
## Library
Add `rustc-demangle = "^0.1.4"` to your Cargo.toml dependencies.
## Executable
Run `cargo install rustc-demangle --features binary` to install,
then pipe output through 'rust-demangle' to demangle it.
For example, `cat asm.S | rust-demangle > asm-demang.S'.

# License

`rustc-demangle` is primarily distributed under the terms of both the MIT license and
the Apache License (Version 2.0), with portions covered by various BSD-like
licenses.

See LICENSE-APACHE, and LICENSE-MIT for details.
