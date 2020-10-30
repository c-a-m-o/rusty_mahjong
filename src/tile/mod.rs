mod suit;
mod dragon;
mod honor;
mod tile_value;

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
    fn new(value : TileValue, id : u8) -> Tile {
        Tile{value, id}
    }

    fn new_suited(suit : Suit, value : u8, id : u8) -> Tile {
        Tile{value : TileValue::new_suited(suit, value), id}
    }

    fn new_wind(wind : Wind, id : u8) -> Tile {
        Tile{value : TileValue::new_wind(wind), id}
    }

    fn new_dragon(dragon : Dragon, id : u8) -> Tile {
        Tile{value : TileValue::new_dragon(dragon), id}
    }

}
