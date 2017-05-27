#[derive(Copy,Clone)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize
}

#[derive(Copy,Clone)]
pub struct Dimension {
    pub width: usize,
    pub height: usize
}

impl Dimension {
    pub fn pixels(self) -> usize {
        return self.width * self.height;
    }
}
