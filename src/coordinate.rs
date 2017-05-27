use std::string::ToString;

#[derive(Copy,Clone)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize
}

impl ToString for Coordinate {
    fn to_string(&self) -> String {
        format!("{} {}", self.x, self.y)
    }
}

#[derive(Copy,Clone)]
pub struct Dimension {
    pub width: usize,
    pub height: usize
}

impl Dimension {
    pub fn null() -> Dimension {
        Dimension {width: 0, height: 0}
    }

    pub fn pixels(self) -> usize {
        self.width * self.height
    }
}

impl Coordinate {
    pub fn null() -> Coordinate {
        Coordinate {x: 0, y: 0}
    }
}
