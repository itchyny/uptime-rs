# Multi-platform uptime library for Rust
[![Travis Build Status](https://travis-ci.org/itchyny/uptime-rs.svg?branch=master)](https://travis-ci.org/itchyny/uptime-rs)
[![AppVeyor Build status](https://ci.appveyor.com/api/projects/status/7osx5jdvs35j58hg?svg=true)](https://ci.appveyor.com/project/itchyny/uptime-rs)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![crates.io](https://img.shields.io/crates/v/uptime_lib.svg)](https://crates.io/crates/uptime_lib)

## Example

```rust
extern crate uptime_lib;

fn main() {
    match uptime_lib::get() {
        Ok(uptime) => {
            println!("uptime: {} seconds", uptime.num_milliseconds() as f64 / 1000.0);
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
