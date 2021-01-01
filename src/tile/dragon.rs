use std::fmt::Debug;
use Dragon::{White, Red, Green};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Dragon {
    White,
    Red,
    Green,
}


impl Dragon {
    pub fn next_dora(&self) -> Dragon {
        match self {
            White => Red,
            Red   => Green,
            Green => White,
        }
    }
}

impl Debug for Dragon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            White => write!(f, "wh"),
            Red => write!(f, "r"),
            Green => write!(f, "g"),
        }
    }
}