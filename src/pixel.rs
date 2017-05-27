use coordinate::Coordinate;
use std::ops::Index;
use std::ops::IndexMut;
use std::option::Option;

#[derive(Copy,Clone)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: Option<u8>,
}

impl Color {
    pub fn null() -> Color {
        Color {red: 0, green: 0, blue: 0, alpha: None}
    }
}

#[derive(Copy,Clone)]
pub struct Pixel{
    pub coordinate : Coordinate,
    pub color: Color,
    pub active: bool
}

impl Pixel {
    pub fn null() -> Pixel {
        Pixel {coordinate: Coordinate::null(), color: Color::null(), active: false}
    }
}

pub fn pixel_command(point: &Pixel) -> String {
    if !point.active {
        return "".to_string();
    }
    let command = format!("PX {} {} {:02x}{:02x}{:02x}\n", point.coordinate.x, point.coordinate.y, point.color.red, point.color.green, point.color.blue);

    return command;
}
