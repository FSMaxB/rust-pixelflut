#[derive(Copy,Clone)]
pub struct Pixel{
    pub x: usize,
    pub y: usize,
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub active: bool
}

pub fn pixel_command(point: &Pixel) -> String {
    if !point.active {
        return "".to_string();
    }
    let command = format!("PX {} {} {:02x}{:02x}{:02x}\n", point.x, point.y, point.red, point.green, point.blue);

    return command;
}
