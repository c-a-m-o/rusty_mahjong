use std::fmt::Display;
use std::fmt::Debug;
use Suit::{Man, Pin, Sou};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit {
    Man,
    Pin,
    Sou,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct SuitedTile {
    suit : Suit,
    value : u8,
}

impl SuitedTile {

    pub fn new(suit : Suit, value : u8) -> SuitedTile {
        if value == 0 || value > 9 {
            println!("Error : creating a SuitedTile with invalid value");
            SuitedTile{suit : Man, value : 0}
        } else {
            SuitedTile{suit, value}
        }
    }

    pub fn value(&self) -> u8 {
        self.value
    } 

    pub fn suit(&self) -> Suit {
        self.suit
    }

    pub fn is_valid(&self) -> bool {
        self.value > 0 && self.value < 9
    }

    pub fn is_terminal(&self) -> bool {
        self.value == 1 || self.value == 9
    }

    pub fn is_simple(&self) -> bool {
        self.value > 1 && self.value < 9
    }
}


impl Display for SuitedTile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.is_valid() {
            write!(f, "{} {:?}", self.value, self.suit)
        } else {
            write!(f, "Invalid tile")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::tile::suit::Suit::{Man, Pin, Sou};
    use crate::tile::suit::SuitedTile;
    
    #[test]
    fn test_valid_tile() {
        let four_pin = SuitedTile::new(Pin, 4);
        assert!(four_pin.is_valid());
    }

    #[test]
    fn test_invalid_tile() {
        let not_valid = SuitedTile::new(Man, 12);
        assert!(!not_valid.is_valid());
        let not_valid_neither = SuitedTile::new(Sou, 0);
        assert!(!not_valid_neither.is_valid());
    }

    #[test]
    fn test_suit_display() {
        let three_man = SuitedTile::new(Man, 3);
        assert_eq!("3 Man", format!("{}", three_man));

        let eight_sou = SuitedTile::new(Sou, 8);
        assert_eq!("8 Sou", format!("{}", eight_sou));
    }

    #[test]
    fn test_suit_order() {
        let three_man = SuitedTile::new(Man, 3);
        let four_man = SuitedTile::new(Man, 4);
        let two_pin = SuitedTile::new(Pin, 2);
        let eight_sou = SuitedTile::new(Sou, 8);
        assert!(three_man < four_man);
        assert!(four_man > three_man);
        assert!(four_man < two_pin);
        assert!(eight_sou > two_pin);
    }
}