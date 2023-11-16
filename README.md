# Multi-platform uptime library for Rust
[![CI Status](https://github.com/itchyny/uptime-rs/actions/workflows/ci.yaml/badge.svg?branch=main)](https://github.com/itchyny/uptime-rs/actions?query=branch:main)
[![crates.io](https://img.shields.io/crates/v/uptime_lib.svg)](https://crates.io/crates/uptime_lib)
[![release](https://img.shields.io/github/release/itchyny/uptime-rs/all.svg)](https://github.com/itchyny/uptime-rs/releases)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/itchyny/uptime-rs/blob/main/LICENSE)

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
