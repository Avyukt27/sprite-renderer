use crate::{colour::Colour, tile::Tile};

#[derive(Clone, Debug)]
pub struct Tilemap {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Tile>,
    pub tile_size: u32,
}

impl Tilemap {
    pub fn new(width: u32, height: u32, tile_size: u32, default_tile: Tile) -> Self {
        Self {
            width,
            height,
            tile_size,
            tiles: vec![default_tile; (width * height) as usize],
        }
    }
    
    pub fn sample(&self, world_x: i32, world_y: i32) -> Option<Colour> {
		if world_x < 0 || world_y < 0 {
			return None;
		}

		let world_x = world_x as u32;
		let world_y = world_y as u32;

		let tile_x = world_x / self.tile_size;
		let tile_y = world_y / self.tile_size;

		if tile_x >= self.width || tile_y >= self.height {
			return None;
		}

		let index = (tile_y * self.width + tile_x) as usize;
		Some(self.tiles[index].colour)
	}
}