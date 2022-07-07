use crate::card::rank::Rank::{self, *};

pub enum DeckType {
    Standard,
    Piquet,
    Jass,
}

use self::DeckType::*;

impl DeckType {
    pub fn ranks(&self) -> Vec<Rank> {
        match self {
            Standard => vec![
                Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King,
            ],
            Piquet => vec![Ace, Seven, Eight, Nine, Ten, Jack, Queen, King],
            Jass => vec![Ace, Six, Seven, Eight, Nine, Ten, Jack, Queen, King],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_ranks() {
        assert_eq!(Standard.ranks().len(), 13);
        assert_eq!(Piquet.ranks().len(), 8);
        assert_eq!(Jass.ranks().len(), 9);
    }
}
