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

fn pixel(x: u16, y: u16, red: u8, green: u8, blue: u8) -> String {
    return format!("PX {} {} {:02x}{:02x}{:02x}", x, y, red, green, blue);
}

fn main() {
    let sleep_time = time::Duration::from_secs(1);
    let tcp_option = TcpStream::connect("94.45.234.7:1234");
    if !tcp_option.is_ok() {
        println!("Failed to open TCP stream.");
        exit(1);
    }
    let mut tcp_stream = tcp_option.unwrap();

    loop {
        for red in 0..255 {
            for green in 0..255 {
                for blue in 0..255 {
                    write_to_stream(&pixel(green as u16 + 100, blue as u16 + 100, red, green, blue), &mut tcp_stream);
                }
            }
        }
    }
}
