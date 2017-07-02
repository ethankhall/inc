fn main() {
    use std::{thread, time};

    let ten_millis = time::Duration::from_millis(1000);
    let now = time::Instant::now();

    thread::sleep(ten_millis);
    println!("Hello world!")
}