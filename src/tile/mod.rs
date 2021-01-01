pub mod suit;
pub mod dragon;
pub mod honor;
pub mod tile_value;

use std::fmt::Debug;

use tile_value::TileValue;
use suit::Suit;
use dragon::Dragon;
use crate::game::wind::Wind;

/// One of the 134 possible riichi mahjong tiles.
/// The variant is stored in the `value` field and two tiles with the same value are separated by their id (there are four of each tile so id is between 0 and 3 included).
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Tile {
    value : TileValue,
    id : u8,
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}({})", self.value, self.id)
    }
}

impl Tile {

    // Constructors

    pub fn new(value : TileValue, id : u8) -> Self {
        Tile{value, id}
    }

    pub fn new_suited(suit : Suit, value : u8, id : u8) -> Self {
        Tile{value : TileValue::new_suited(suit, value), id}
    }

    pub fn new_wind(wind : Wind, id : u8) -> Self {
        Tile{value : TileValue::new_wind(wind), id}
    }

    pub fn new_dragon(dragon : Dragon, id : u8) -> Self {
        Tile{value : TileValue::new_dragon(dragon), id}
    }

    // Immutable getters

    pub fn value(&self) -> TileValue {
        self.value
    }

    pub fn id(&self) -> u8 {
        self.id
    }

    // Other

    /// Gets the dora tile if this tile is the dora indicator
    pub fn next_dora(&self) -> TileValue {
        self.value.next_dora()
    }

    /// If the tile is a suited tile and its value is lower than 9, gets the value of the next tile
    pub fn next(&self) -> Option<TileValue> {
        self.value.next()
    }

    /// If the tile is a suited tile and its value is greater than 1, gets the value of the previous tile
    pub fn prev(&self) -> Option<TileValue> {
        self.value.prev()
    }

    /// Returns whether the `other` tile is the tile just after this one
    pub fn followed_by(&self, other : Tile) -> bool {
        self.value.followed_by(other.value)
    }

    /// If this tile is a suited tile, returns the number indicated by this tile.
    /// Else, returns `None`
    pub fn number(&self) -> Option<u8> {
        match self.value {
            TileValue::Suited(suited) => Some(suited.value()),
            TileValue::Honor(_) => None,
        }
    }

}
