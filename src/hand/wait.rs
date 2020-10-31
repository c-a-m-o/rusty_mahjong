use crate::tile::Tile;
use crate::tile::tile_value::TileValue;

/// Represents a group of tiles where one tile is missing.
/// It is used to know on what tiles a hand is waiting.
#[derive(Clone, Debug)]
pub struct Wait {
    tiles : Vec<Tile>,
    wait_symbols : Vec<TileValue>,
}

impl Wait {

    pub fn new(tiles : Vec<Tile>, wait_symbols : Vec<TileValue>) -> Wait {
        Wait{tiles, wait_symbols}
    }

    pub fn tiles(&self) -> &Vec<Tile> {
        &self.tiles
    }

    pub fn wait_symbols(&self) -> &Vec<TileValue> {
        &self.wait_symbols
    }
}