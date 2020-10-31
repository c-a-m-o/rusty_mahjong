use crate::tile::Tile;
use crate::hand::group::Group;
use crate::game::wind::Wind;

/// Represents an open group, where one of the tiles comes
/// from another player's hand
#[derive(Debug)]
pub struct Meld {
    group : Group,
    completing_tile : Tile, // The tile that comes from another player's discord 
    source : Wind, // The player that discarded the completing tile
}

impl Meld {
    pub fn tiles(&self) -> Vec<&Tile> {
        self.group.tiles()
    }
}