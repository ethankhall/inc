#[macro_use]
extern crate slog;
extern crate etrain;

use etrain::logging::{logging, log_from_env};

fn main() {
    let logger = logging(log_from_env(), "etrain-checkout");
    slog_debug!(logger, "Starting checkout");

    use std::{thread, time};

    let ten_millis = time::Duration::from_millis(1000);

    thread::sleep(ten_millis);
    println!("Hello world!")
}
