pub mod suit;
pub mod dragon;
pub mod honor;
pub mod tile_value;

use tile_value::TileValue;
use suit::Suit;
use dragon::Dragon;
use crate::game::wind::Wind;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Tile {
    value : TileValue,
    id : u8,
}

impl Tile {
    pub fn new(value : TileValue, id : u8) -> Tile {
        Tile{value, id}
    }

    pub fn new_suited(suit : Suit, value : u8, id : u8) -> Tile {
        Tile{value : TileValue::new_suited(suit, value), id}
    }

    pub fn new_wind(wind : Wind, id : u8) -> Tile {
        Tile{value : TileValue::new_wind(wind), id}
    }

    pub fn new_dragon(dragon : Dragon, id : u8) -> Tile {
        Tile{value : TileValue::new_dragon(dragon), id}
    }

    pub fn value(&self) -> TileValue {
        self.value
    }

    pub fn next_dora(&self) -> TileValue {
        self.value.next_dora()
    }

    pub fn followed_by(&self, other : Tile) -> bool {
        self.value.followed_by(other.value)
    }

    pub fn next(&self) -> Option<TileValue> {
        self.value.next()
    }

    pub fn prev(&self) -> Option<TileValue> {
        self.value.prev()
    }

    pub fn number(&self) -> Option<u8> {
        match self.value {
            TileValue::Suited(suited) => Some(suited.value()),
            TileValue::Honor(_) => None,
        }
    }

}
