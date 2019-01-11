# platform crate [![Crates listing](https://img.shields.io/crates/v/platform.svg)](https://crates.io/crates/platform) [![Travis](https://travis-ci.com/Xaeroxe/platform.svg?branch=master)](https://travis-ci.com/Xaeroxe/platform)

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
        .wasm32("wasm32")
        .emscripten("emscripten")
    );
}
```

# Contributing

I welcome contributions and alterations to this project! [Here's some info to help you get started.](https://help.github.com/articles/about-pull-requests/)

- If you would consider your change substantial open an issue on the issues tab so we can discuss it before you build it.
- If you're fixing a bug please provide a unit test for the bug fixed so we don't do it again.
- If applicable, new features should have some basic unit tests added too.
- Please format your code with the most recent stable release of rustfmt before submitting your PR.
- I don't have a regular release schedule, if you want something you've added put on crates.io right away make sure to
bump the version number for the project in your pull request.
