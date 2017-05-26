use std::{thread,time};
use std::io::prelude::*;
use std::net::TcpStream;
use std::process::exit;

fn write_to_stream(line: &String, stream: &mut TcpStream) -> bool {
    let mut written;
    let mut result = stream.write(line.as_bytes());
    if !result.is_ok() {
        return false;
    } else {
        written = result.unwrap();
    }

    result = stream.write(b"\n");
    if !result.is_ok() {
        return false;
    } else {
        written += result.unwrap();
    }

    if written != (line.len() + 1) {
        return false;
    }

    return true;
}

fn main() {
    let sleep_time = time::Duration::from_secs(1);
    let tcp_option = TcpStream::connect("localhost:8080");
    if !tcp_option.is_ok() {
        println!("Failed to open TCP stream.");
        exit(1);
    }
    let mut tcp_stream = tcp_option.unwrap();
    let mut counter = 0;

    loop {
        let number = format!("{}", counter);
        write_to_stream(&number, &mut tcp_stream);
        counter = counter + 1;
        thread::sleep(sleep_time);
    }
}
