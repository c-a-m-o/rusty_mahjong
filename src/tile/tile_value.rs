use super::suit::SuitedTile;
use super::suit::Suit;
use super::honor::HonorTile;
use super::dragon::Dragon;
use crate::game::wind::Wind;
use TileValue::{Suited, Honor};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum TileValue {
    Suited(SuitedTile),
    Honor(HonorTile),
}

impl TileValue {

    pub fn from_suited(tile : SuitedTile) -> TileValue {
        Suited(tile)
    }

    pub fn from_honor(tile : HonorTile) -> TileValue {
        Honor(tile)
    }

    pub fn new_suited(suit : Suit, value : u8) -> TileValue {
        Suited(SuitedTile::new(suit, value))
    }

    pub fn new_dragon(dragon : Dragon) -> TileValue {
        Honor(HonorTile::Dragon(dragon))
    }

    pub fn new_wind(wind : Wind) -> TileValue {
        Honor(HonorTile::Wind(wind))
    }


    pub fn next_dora(&self) -> TileValue {
        match self {
            Suited(suited) => Suited(suited.next_dora()),
            Honor(honor)   => Honor(
                match honor {
                    HonorTile::Wind(wind) => HonorTile::Wind(wind.next_dora()),
                    HonorTile::Dragon(dragon) => HonorTile::Dragon(dragon.next_dora()),
                }
            ),
        }
    }

    pub fn next(&self) -> Option<TileValue> {
        match self {
            Suited(suited) => suited.next().map(|x|{Suited(x)}),
            Honor(_)       => None,
        }
    }

    pub fn prev(&self) -> Option<TileValue> {
        match self {
            Suited(suited) => suited.prev().map(|x|{Suited(x)}),
            Honor(_)       => None,
        }
    }

    pub fn followed_by(&self, other : TileValue) -> bool {
        if let Suited(self_suited) = self {
            if let Suited(other_suited) = other {
                return self_suited.followed_by(other_suited);
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {

    use super::TileValue;
    use crate::tile::suit::Suit::{Man, Sou};
    use crate::tile::dragon::Dragon::{White, Red};
    use crate::game::wind::Wind::{East, South, West, North};

    #[test]
    fn test_next_dora() {
        assert_eq!(TileValue::new_suited(Man, 4), TileValue::new_suited(Man, 3).next_dora());
        assert_eq!(TileValue::new_dragon(Red), TileValue::new_dragon(White).next_dora());
        assert_eq!(TileValue::new_wind(West), TileValue::new_wind(South).next_dora());

        assert_eq!(TileValue::new_suited(Sou, 1), TileValue::new_suited(Sou, 9).next_dora());
        assert_eq!(TileValue::new_wind(East), TileValue::new_wind(North).next_dora())
    }
}