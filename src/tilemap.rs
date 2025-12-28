use crate::tile::Tile;

#[derive(Clone, Debug)]
pub struct Tilemap {
    pub width: u32,
    pub height: u32,
    pub tiles: Vec<Tile>,
}