use Wind::{East, South, West, North};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Wind {
    East,
    South,
    West,
    North,
}

impl Wind {
    pub fn next_dora(&self) -> Wind {
        match self {
            East  => South,
            South => West,
            West  => North,
            North => East,
        }
    }

    pub fn next(&self) -> Option<Wind> {
        match self {
            East  => Some(South),
            South => Some(West),
            West  => Some(North),
            North => None,
        }
    }
}