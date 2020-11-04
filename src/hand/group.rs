use crate::tile::Tile;

/// Represents a closed group in a mahjong hand.
#[derive(Clone, Debug)]
pub enum Group {
    Pon(Tile, Tile, Tile),
    Chii(Tile, Tile, Tile),
    Pair(Tile, Tile),
    Kan(Tile, Tile, Tile, Tile),
}

impl Group {
    pub fn tiles(&self) -> Vec<&Tile> {
        match self {
            Group::Pair(x, y) => vec![x, y],
            | Group::Pon(x, y, z)
            | Group::Chii(x, y, z) => vec![x, y, z],
            Group::Kan(x, y, z, t) => vec![x, y, z, t],
        }
    }
}