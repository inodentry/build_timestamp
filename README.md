# Build Timestamp String Generator

`build_timestamp` is a simple proc-macro for Rust to generate a `const` string
with a build timestamp (in UTC), formatted with `strftime` syntax.

Note: using this crate makes your build non-deterministic!

Note: using this crate requires nightly Rust

## Example

```rust
// Rust2018 macro import
use build_timestamp::build_time;

// generates a `const BUILD_TIME: &str`
build_time!("%A %Y-%m-%d / %H:%M:%S");

fn main() {
    println!("This is {}, built on: {}",
        env!("CARGO_PKG_NAME"), BUILD_TIME);
}
```

Running this example will print something that looks like:

```
This is my_crate, built on: Saturday 2018-09-08 / 11:35:43
```

## TODO

Features that I'd like this crate to support in the future:
  - proper error handling (currently just uses `unwrap()` to panic)
  - ability to specify the name for the generated `const`
  - non-UTC timezones

If you would like to implement any of this, I welcome patches.

