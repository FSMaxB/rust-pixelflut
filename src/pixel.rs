use coordinate::Coordinate;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Copy,Clone)]
pub struct Pixel{
    pub coordinate : Coordinate,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub active: bool
}

impl Pixel {
    pub fn null() -> Pixel {
        Pixel {coordinate: Coordinate::null(), red: 0, green: 0, blue: 0, active: false}
    }
}

pub fn pixel_command(point: &Pixel) -> String {
    if !point.active {
        return "".to_string();
    }
    let command = format!("PX {} {} {:02x}{:02x}{:02x}\n", point.coordinate.x, point.coordinate.y, point.red, point.green, point.blue);

    return command;
}
