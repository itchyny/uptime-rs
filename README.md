# Multi-platform uptime library for Rust
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
