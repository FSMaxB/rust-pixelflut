use coordinate::Coordinate;
use std::ops::Index;
use std::ops::IndexMut;
use std::option::Option;
use std::string::ToString;

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

    pub fn gray(color: u8) -> Color {
        Color {red: color, green: color, blue: color, alpha: None}
    }

    pub fn new(red: u8, green: u8, blue: u8) -> Color {
        Color {red: red, green: green, blue: blue, alpha: None}
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        match self.alpha {
            Some(u8) => format!("{:02x}{:02x}{:02x}{:02x}", self.red, self.green, self.blue, self.alpha.unwrap()),
            None => format!("{:02x}{:02x}{:02x}", self.red, self.green, self.blue),
        }
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
    let command = format!("PX {} {} {}\n", point.coordinate.x, point.coordinate.y, point.color.to_string());

    return command;
}
