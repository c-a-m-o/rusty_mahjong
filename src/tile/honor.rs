use crate::game::wind::Wind;
use crate::tile::dragon::Dragon;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum HonorTile {
    Wind(Wind),
    Dragon(Dragon),
}