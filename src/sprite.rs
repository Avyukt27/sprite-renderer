use crate::colour::Colour;

#[derive(Clone, Copy, Debug)]
pub struct Sprite {
    pub x: i32,
    pub y: i32,
    pub colour: Colour,
    pub width: u32,
    pub height: u32,
}

impl Sprite {
    pub fn new(x: i32, y: i32, colour: Colour, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            colour,
            width,
            height,
        }
    }
}
