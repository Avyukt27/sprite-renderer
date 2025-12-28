use crate::{colour::Colour, sprite::Sprite, tilemap::Tilemap};

pub struct Renderer {
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<u8>,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; (width * height * 4) as usize],
        }
    }

    pub fn clear(&mut self, colour: Colour) {
        for pixel in self.buffer.chunks_exact_mut(4) {
            pixel[0] = colour.r;
            pixel[1] = colour.g;
            pixel[2] = colour.b;
            pixel[3] = colour.a;
        }
    }

    pub fn draw_sprite(&mut self, sprite: &Sprite) {
        if !sprite.visible {
            return;
        }

        for sprite_y in 0..sprite.height {
            for sprite_x in 0..sprite.width {
                let x = sprite.x + sprite_x as i32;
                let y = sprite.y + sprite_y as i32;

                if x < 0 || y < 0 {
                    continue;
                }

                let x = x as u32;
                let y = y as u32;

                if x >= self.width || y >= self.height {
                    continue;
                }

                let index = ((y * self.width + x) * 4) as usize;

                self.buffer[index] = sprite.colour.r;
                self.buffer[index + 1] = sprite.colour.g;
                self.buffer[index + 2] = sprite.colour.b;
                self.buffer[index + 3] = sprite.colour.a;
            }
        }
    }

    pub fn draw_background(&mut self, tilemap: &Tilemap) {
        for tile_y in 0..tilemap.height {
            for tile_x in 0..tilemap.width {
                let tile_index = (tile_y * tilemap.width + tile_x) as usize;
                let tile = &tilemap.tiles[tile_index];

                let pixel_x_start = tile_x * 8;
                let pixel_y_start = tile_y * 8;

                for y in 0..8 {
                    for x in 0..8 {
                        let pixel_x = pixel_x_start + x;
                        let pixel_y = pixel_y_start + y;

                        if pixel_x >= self.width || pixel_y >= self.height {
                            continue;
                        }

                        let index = ((pixel_y * self.width + pixel_x) * 4) as usize;

                        self.buffer[index] = tile.colour.r;
                        self.buffer[index + 1] = tile.colour.g;
                        self.buffer[index + 2] = tile.colour.b;
                        self.buffer[index + 3] = tile.colour.a;
                    }
                }
            }
        }
    }
}
