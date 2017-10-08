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
