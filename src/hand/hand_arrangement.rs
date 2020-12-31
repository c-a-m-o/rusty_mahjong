use crate::hand::group::Group;
use crate::hand::wait::Wait;
use crate::tile::Tile;
use crate::tile::tile_value::TileValue;

#[derive(Clone, Debug)]
pub struct HandArrangement {
    pub groups : Vec<Group>,
    pub wait : Option<Wait>,
}

type ExtractionFunction = fn(&HandArrangement, tiles : &Vec<Tile>) -> Option<(HandArrangement, Vec<Tile>)>;

impl HandArrangement {

    pub fn empty() -> HandArrangement {
        HandArrangement {
            groups : Vec::new(),
            wait : None,
        }
    }

    pub fn groups(&self) -> &Vec<Group> {
        &self.groups
    }

    pub fn wait(&self) -> &Option<Wait> {
        &self.wait
    }

    /// Recursive function that determines the possible waiting arrangements that can be
    /// made with the tiles contained in the closed part of a hand.
    /// The `hidden_tiles` vector has to be sorted beforehand.
    fn consume(&self, hidden_tiles : &Vec<Tile>) -> Vec<HandArrangement> {

        let mut arrangements : Vec<HandArrangement> = Vec::new();
        
        if hidden_tiles.is_empty() {
            if self.wait.is_some() {
                arrangements.push(self.clone());
            }
            return arrangements;
        }

        self.extract(hidden_tiles, &mut arrangements, HandArrangement::extract_pon);
        self.extract(hidden_tiles, &mut arrangements, HandArrangement::extract_pair);
        self.extract(hidden_tiles, &mut arrangements, HandArrangement::extract_chii);

        if self.wait.is_none() {
            self.extract(hidden_tiles, &mut arrangements, HandArrangement::extract_pon_wait);
            self.extract(hidden_tiles, &mut arrangements, HandArrangement::extract_chii_side_wait);
            self.extract(hidden_tiles, &mut arrangements, HandArrangement::extract_chii_middle_wait);
            self.extract(hidden_tiles, &mut arrangements, HandArrangement::extract_pair_wait);
        }

        arrangements
    }
    
    fn extract(
        &self,
        hidden_tiles : &Vec<Tile>,
        arrangements : &mut Vec<HandArrangement>,
        function : ExtractionFunction,
    ) {
        if let Some((new_arrangement, remaining_tiles)) = function(&self, hidden_tiles) {
            arrangements.append(&mut new_arrangement.consume(&remaining_tiles));
        }
    }

    fn extract_pon(&self, hidden_tiles : &Vec<Tile>) -> Option<(HandArrangement, Vec<Tile>)> {
        if hidden_tiles.len() < 3 {
            return None;
        }

        let mut hidden_tiles = hidden_tiles.clone();

        let first  = hidden_tiles.remove(0);
        let second = hidden_tiles.remove(0);
        let third  = hidden_tiles.remove(0);

        if first.value() == second.value() && first.value() == third.value() {

            let mut groups = self.groups.clone();

            groups.push(Group::Pon(first, second, third));

            Some((
                HandArrangement{
                    groups,
                    wait : self.wait.clone(),
                },
                hidden_tiles
            ))

        } else {
            None
        }
    }

    fn extract_pair(&self, hidden_tiles : &Vec<Tile>) -> Option<(HandArrangement, Vec<Tile>)> {
        if hidden_tiles.len() < 2 {
            return None;
        }
        // If the arrangement already contains a pair
        if self.groups.iter().any(|group| {if let Group::Pair(_,_) = group {true} else {false}}) {
            return None;
        }

        let mut hidden_tiles = hidden_tiles.clone();

        let first  = hidden_tiles.remove(0);
        let second = hidden_tiles.remove(0);

        if first.value() == second.value() {

            let mut groups = self.groups.clone();
            groups.push(Group::Pair(first, second));

            Some((
                HandArrangement{
                    groups,
                    wait : self.wait.clone(),
                },
                hidden_tiles
            ))

        } else {
            None
        }

    }

    fn extract_chii(&self, hidden_tiles : &Vec<Tile>) -> Option<(HandArrangement, Vec<Tile>)> {
        if hidden_tiles.len() < 3 {
            return None;
        }
        let mut i = 0; // index of the removed tile
        let mut hidden_tiles = hidden_tiles.clone();
        // --- First tile
        let first  = hidden_tiles.remove(0);
        if let TileValue::Honor(_) = first.value() {
            return None;
        }

        // --- Second tile
        // Advances to the first different tile
        while hidden_tiles[i].value() == first.value() {
            i += 1;
            // The -1 is because we need to be able to extract a third tile
            if i == hidden_tiles.len() - 1 {
                return None;
            }
        }
        let second = hidden_tiles.remove(i);

        if !first.followed_by(second) {
            return None;
        }

        // --- Third tile
        // Advances to the first different tile
        while hidden_tiles[i].value() == second.value() {
            i += 1;
            if i == hidden_tiles.len() {
                return None;
            }
        }
        let third  = hidden_tiles.remove(i);

        if !second.followed_by(third) {
            return None;
        }

        let mut groups = self.groups.clone();
        groups.push(Group::Chii(first, second, third));

        Some ((
            HandArrangement{
                groups,
                wait : self.wait.clone(),
            },
            hidden_tiles,
        ))

    }

    fn extract_pon_wait(&self, hidden_tiles : &Vec<Tile>) -> Option<(HandArrangement, Vec<Tile>)> {
        // TODO : consider mergeing it with extract_pair
        if hidden_tiles.len() < 2 {
            return None;
        }

        let mut hidden_tiles = hidden_tiles.clone();

        let first  = hidden_tiles.remove(0);
        let second = hidden_tiles.remove(0);

        if first.value() == second.value() {
            let wait = Some(Wait::new(vec![first, second], vec![first.value()]));
            Some((
                HandArrangement{
                    groups : self.groups.clone(),
                    wait,
                },
                hidden_tiles
            ))
        } else {
            None
        }
    }

    fn extract_pair_wait(&self, hidden_tiles : &Vec<Tile>) -> Option<(HandArrangement, Vec<Tile>)> {
        if hidden_tiles.len() < 1 {
            return None;
        }
        // If the arrangement already contains a pair
        if self.groups.iter().any(|group| {if let Group::Pair(_,_) = group {false} else {true}}) {
            return None;
        }

        let mut hidden_tiles = hidden_tiles.clone();
        let tile = hidden_tiles.remove(0);

        let wait = Some(Wait::new(vec![tile], vec![tile.value()]));
        Some((
            HandArrangement {
                groups: self.groups.clone(),
                wait,
            },
            hidden_tiles,
        ))
    }

    fn extract_chii_side_wait(&self, hidden_tiles : &Vec<Tile>) -> Option<(HandArrangement, Vec<Tile>)> {
        if hidden_tiles.len() < 2 {
            return None;
        }
        let mut hidden_tiles = hidden_tiles.clone();

        let mut i = 0;
        let first = hidden_tiles.remove(0);

        if let TileValue::Honor(_) = first.value() {
            return None;
        }

        while hidden_tiles[i].value() == first.value() {
            i += 1;
            if i == hidden_tiles.len() {
                return None;
            }
        }

        let second = hidden_tiles.remove(0);

        if first.followed_by(second) {
            let wait_symbols : Vec<TileValue> = vec![first.prev(), second.next()]
                .into_iter()
                .filter(|x| x.is_some())
                .map(Option::unwrap)
                .collect();
            let wait = Some(Wait::new(vec![first, second], wait_symbols));

            Some((
                HandArrangement{
                    groups : self.groups.clone(),
                    wait,
                },
                hidden_tiles,
            ))
        } else {
            None
        }
    }

    fn extract_chii_middle_wait(&self, hidden_tiles : &Vec<Tile>) -> Option<(HandArrangement, Vec<Tile>)> {
        if hidden_tiles.len() < 2 {
            return None;
        }

        let mut hidden_tiles = hidden_tiles.clone();
        let first = hidden_tiles.remove(0);

        if let TileValue::Honor(_) = first.value() {
            return None;
        }

        let third_symbol = first.next()?.next()?;

        let mut i = 0;
        while hidden_tiles[i].value() < third_symbol {
            i += 1;
            if i == hidden_tiles.len() {
                return None;
            }
        }

        if hidden_tiles[i].value() == third_symbol {
            let third = hidden_tiles.remove(i);

            let wait = Some(Wait::new(vec![first, third], vec![first.next()?]));

            Some((
                HandArrangement{
                    groups : self.groups.clone(),
                    wait,
                },
                hidden_tiles
            ))
        } else {
            None
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::tile::Tile;
    use crate::tile::suit::Suit;
    use crate::tile::dragon::Dragon;
    use crate::hand::hand_arrangement::HandArrangement;
    // TODO : assertions instead of prints
    
    #[test]
    fn test_pair_wait() {
        let arrangement = HandArrangement::empty();
        let tiles = vec![
            Tile::new_dragon(Dragon::White, 0),
        ];
        println!("{:?}", arrangement.consume(&tiles));
    }


    #[test]
    fn test_pon_wait() {
        let arrangement = HandArrangement::empty();
        let tiles = vec![
            Tile::new_dragon(Dragon::White, 0),
            Tile::new_dragon(Dragon::White, 1),
        ];
        println!("{:?}", arrangement.consume(&tiles));
    }

    #[test]
    fn test_pon() {
        let arrangement = HandArrangement::empty();
        let tiles = vec![
            Tile::new_suited(Suit::Man, 3, 0),
            Tile::new_suited(Suit::Man, 3, 1),
            Tile::new_suited(Suit::Man, 3, 2),
            Tile::new_dragon(Dragon::White, 0),
            Tile::new_dragon(Dragon::White, 1),
        ];
        println!("{:?}", arrangement.consume(&tiles));
    }

    #[test]
    fn test_chii() {
        let arrangement = HandArrangement::empty();
        let tiles = vec![
            Tile::new_suited(Suit::Man, 3, 0),
            Tile::new_suited(Suit::Man, 4, 0),
            Tile::new_suited(Suit::Man, 5, 0),
            Tile::new_dragon(Dragon::White, 0),
            Tile::new_dragon(Dragon::White, 1),
        ];
        println!("{:?}", arrangement.consume(&tiles));
    }

    #[test]
    fn test_chii2() {
        let arrangement = HandArrangement::empty();
        let tiles = vec![
            Tile::new_suited(Suit::Man, 3, 0),
            Tile::new_suited(Suit::Man, 4, 0),
            Tile::new_suited(Suit::Man, 4, 1),
            Tile::new_suited(Suit::Man, 4, 2),
            Tile::new_suited(Suit::Man, 5, 0),
            Tile::new_dragon(Dragon::White, 0),
            Tile::new_dragon(Dragon::White, 1),
        ];
        println!("{:?}", arrangement.consume(&tiles).len());
    }

    #[test]
    fn test2() {
        let arrangement = HandArrangement::empty();
        let tiles4 = vec![
            Tile::new_suited(Suit::Man, 3, 0),
            Tile::new_suited(Suit::Man, 3, 1),
            Tile::new_suited(Suit::Man, 3, 2),
            Tile::new_suited(Suit::Man, 4, 0),
            Tile::new_suited(Suit::Man, 4, 1),
            Tile::new_suited(Suit::Man, 4, 2),
            Tile::new_suited(Suit::Man, 5, 0),
            Tile::new_suited(Suit::Man, 5, 1),
            Tile::new_suited(Suit::Man, 5, 2),
            Tile::new_dragon(Dragon::White, 0),
            Tile::new_dragon(Dragon::White, 1),
        ];
        println!("{:?}", arrangement.consume(&tiles4));
    }

    #[test]
    fn test_pinfu() {
        let arrangement = HandArrangement::empty();
        let tiles = vec![
            Tile::new_suited(Suit::Man, 3, 0),
            Tile::new_suited(Suit::Man, 4, 0),
            Tile::new_suited(Suit::Pin, 4, 0),
            Tile::new_suited(Suit::Pin, 5, 0),
            Tile::new_suited(Suit::Pin, 5, 1),
            Tile::new_suited(Suit::Pin, 5, 2),
            Tile::new_suited(Suit::Pin, 6, 0),
            Tile::new_suited(Suit::Pin, 6, 1),
            Tile::new_suited(Suit::Pin, 7, 0),
            Tile::new_suited(Suit::Pin, 8, 0),
            Tile::new_suited(Suit::Sou, 3, 0),
            Tile::new_suited(Suit::Sou, 4, 0),
            Tile::new_suited(Suit::Sou, 5, 0),
        ];

        println!("{:?}", arrangement.consume(&tiles));
    }
}