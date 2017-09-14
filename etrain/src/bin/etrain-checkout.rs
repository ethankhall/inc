#[macro_use]
extern crate slog;
extern crate etrain;

use etrain::initialization::{logging, log_from_env};

fn main() {
    let logger = logging(log_from_env(), "etrain");
    slog_debug!(logger, "Starting checkout");

    use std::{thread, time};

    let ten_millis = time::Duration::from_millis(1000);
    let now = time::Instant::now();

    thread::sleep(ten_millis);
    println!("Hello world!")
}