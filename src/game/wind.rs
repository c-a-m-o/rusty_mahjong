use std::fmt::Debug;
use Wind::{East, South, West, North};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
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

impl Debug for Wind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            East => write!(f, "e"),
            South => write!(f, "s"),
            West => write!(f, "w"),
            North => write!(f, "n"),
        }
    }
}