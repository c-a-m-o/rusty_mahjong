use Dragon::{White, Red, Green};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
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