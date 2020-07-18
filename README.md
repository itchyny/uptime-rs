# Multi-platform uptime library for Rust [![CI Status](https://github.com/itchyny/uptime-rs/workflows/CI/badge.svg)](https://github.com/itchyny/uptime-rs/actions) [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE) [![crates.io](https://img.shields.io/crates/v/uptime_lib.svg)](https://crates.io/crates/uptime_lib)

## Example

```rust
extern crate uptime_lib;

fn main() {
    match uptime_lib::get() {
        Ok(uptime) => {
            println!("uptime: {} seconds", uptime.as_secs_f64());
        }
        Err(err) => {
            eprintln!("uptime: {}", err);
            std::process::exit(1);
        }
    }
}
```

## Author
itchyny (https://github.com/itchyny)

## License
This software is released under the MIT License, see LICENSE.
