use crate::card::Rank::{self, *};
use crate::deck::DeckType::{self, *};

impl DeckType {
    pub fn ranks(&self) -> Vec<Rank> {
        match self {
            Standard => vec![Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King],
            Piquet => vec![Ace, Seven, Eight, Nine, Ten, Jack, Queen, King],
            Jass => vec![Ace, Six, Seven, Eight, Nine, Ten, Jack, Queen, King],
        }
    }

    pub fn decksize(&self) -> usize {
        4 * self.ranks().len()
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
