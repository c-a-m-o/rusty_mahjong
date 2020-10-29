mod suit;

use suit::SuitedTile;

pub enum TileValue {
    Suit(SuitedTile),
    Honor,
}