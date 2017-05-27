use coordinate::Coordinate;
use coordinate::Dimension;
use std::ops::Index;
use std::ops::IndexMut;
use std::option::Option;
use std::string::ToString;
extern crate rand;
use rand::Rng;

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

impl ToString for Pixel {
    fn to_string(&self) -> String {
        match self.active {
            true => format!("PX {} {}\n", self.coordinate.to_string(), self.color.to_string()),
            false => "".to_string()
        }
    }
}

pub struct Field {
    field : Vec<Vec<Pixel>>,
    dimension: Dimension,
}

impl Field {
    pub fn new(dimension: Dimension) -> Field {
        Field {
            field: vec![vec![Pixel::null(); dimension.height]; dimension.width],
            dimension: dimension,
        }
    }

    pub fn dimension(&self) -> &Dimension {
        &self.dimension
    }

    pub fn serialise(&self) -> Vec<Pixel> {
        let pixels = self.dimension.pixels();
        let mut serialised = vec![Pixel::null(); pixels];
        for index in 0..pixels {
            let x = index % self.dimension.width;
            let y = index / self.dimension.width;

            serialised[index] = self.field[x][y];
        }

        let mut rng = rand::thread_rng();
        rng.shuffle(&mut serialised[..]);

        return serialised;
    }
}

impl Index<usize> for Field {
    type Output = Vec<Pixel>;
    fn index(&self, index: usize) -> &Vec<Pixel> {
        &self.field[index]
    }
}

impl IndexMut<usize> for Field {
    fn index_mut(&mut self, index: usize) -> &mut Vec<Pixel> {
        &mut self.field[index]
    }
}
