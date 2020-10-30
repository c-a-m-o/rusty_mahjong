mod suit;
mod dragon;
mod honor;

use suit::SuitedTile;
use honor::Honor;

pub enum TileValue {
    Suit(SuitedTile),
    Honor(Honor),
}