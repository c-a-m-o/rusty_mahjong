use std::fmt::Debug;

use crate::game::wind::Wind;
use crate::tile::dragon::Dragon;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum HonorTile {
    Wind(Wind),
    Dragon(Dragon),
}

impl Debug for HonorTile {
    fn fmt(&self, f : &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HonorTile::Wind(wind) => write!(f, "{:?}", &wind),
            HonorTile::Dragon(dragon) => write!(f, "{:?}", &dragon)
        }
    }
}