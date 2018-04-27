[![Crates listing](https://img.shields.io/crates/v/platform.svg)](https://crates.io/crates/platform)

# platform crate

This crate provides an easy way to inline selection of input parameters
based on the platform being targeted.  Can be used on any `Sized` type.

This is guaranteed to be a zero cost abstraction, as all calls are inlined.

```rust
extern crate platform;

use platform::Platform;

fn main() {
    println!("Hello from {}!", 
        "unknown"
        .ios("ios")
        .android("android")
        .windows("windows")
        .macos("macos")
        .linux("linux")
        .emscripten("emscripten")
    );
}
```