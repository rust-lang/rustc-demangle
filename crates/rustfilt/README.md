[![main](https://github.com/luser/rustfilt/actions/workflows/main.yml/badge.svg)](https://github.com/luser/rustfilt/actions/workflows/main.yml)
[![Crates.io](https://img.shields.io/crates/v/rustfilt.svg)](https://crates.io/crates/rustfilt)

Demangle Rust symbol names using [rustc-demangle](https://github.com/alexcrichton/rustc-demangle). `rustfilt` works similarly to `c++filt`, in that it accepts mangled symbol names as command line arguments, and if none are provided it accepts mangled symbols from stdin. Demangled symbols are written to stdout.

## Installation
````bash
cargo install rustfilt
````

## Usage
To demangle a file, simply run:
````bash
rustfilt -i mangled.txt -o demangled.txt
````
Rustfilt can also accept data from stdin, and pipe to stdout:
````
curl http://example.com/mangled-symbols.txt | rustfilt | less
````

By default, rustfilt strips the generated 'hashes' from the mangled names.
If these need to be kept, simply pass the `-h` option to rustfilt.
