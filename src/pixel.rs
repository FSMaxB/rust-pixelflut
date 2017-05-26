#[derive(Copy,Clone)]
pub struct Point {
    pub x: usize,
    pub y: usize,
    pub red: u8,
    pub green: u8,
    pub blue: u8
}

pub fn pixel_command(point: &Point) -> String {
    let command = format!("PX {} {} {:02x}{:02x}{:02x}\n", point.x, point.y, point.red, point.green, point.blue);

    return command;
}
