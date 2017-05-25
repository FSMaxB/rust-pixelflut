use std::{thread,time};

fn main() {
    let sleep_time = time::Duration::from_secs(1);
    loop {
        println!("Hello, world!");
        thread::sleep(sleep_time);
    }
}
