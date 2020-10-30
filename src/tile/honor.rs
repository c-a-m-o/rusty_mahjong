use crate::game::wind::Wind;
use crate::tile::dragon::Dragon;

pub enum Honor {
    Wind(Wind),
    Dragon(Dragon)
}