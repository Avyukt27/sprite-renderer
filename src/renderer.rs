use crate::{colour::Colour, sprite::Sprite, tilemap::Tilemap};

pub struct Renderer {
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<u8>,
    pub scroll_x: i32,
    pub scroll_y: i32,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            buffer: vec![0; (width * height * 4) as usize],
            scroll_x: 0,
            scroll_y: 0,
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
        for screen_y in 0..self.height {
            for screen_x in 0..self.width {
                let world_x = screen_x as i32 + self.scroll_x;
                let world_y = screen_y as i32 + self.scroll_y;

                let mut tile_x = world_x.div_euclid(8);
                let mut tile_y = world_y.div_euclid(8);

                tile_x = tile_x.rem_euclid(tilemap.width as i32);
                tile_y = tile_y.rem_euclid(tilemap.height as i32);

                let tile = tilemap.tiles[(tile_y as u32 * tilemap.width + tile_x as u32) as usize];

                let index = ((screen_y * self.width + screen_x) * 4) as usize;

                self.buffer[index] = tile.colour.r;
                self.buffer[index + 1] = tile.colour.g;
                self.buffer[index + 2] = tile.colour.b;
                self.buffer[index + 3] = tile.colour.a;
            }
        }
    }
}
