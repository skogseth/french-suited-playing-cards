pub mod rank;
pub mod suit;
use crate::card::{rank::Rank, suit::Suit};

/// A card with a specified rank and suit.
///
/// # Examples
///
/// ```
/// use cards::{Card, Rank, Suit};
///
/// let card = Card::new(Rank::Ace, Suit::Spades);
/// assert_eq!(card.rank(), Rank::Ace);
/// assert_eq!(card.suit(), Suit::Spades);
/// assert_eq!(format!("{card}"), "Ace of Spades");
/// ```
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Card {
        Card { rank, suit }
    }

    pub fn rank(&self) -> Rank {
        self.rank
    }
    pub fn suit(&self) -> Suit {
        self.suit
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} of {}", self.rank, self.suit)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print() {
        let mut card;
        for r in Rank::iterator() {
            for s in Suit::iterator() {
                card = Card::new(r, s);
                println!("{}", card);
            }
        }
    }

    #[test]
    fn display() {
        let card = Card::new(Rank::Ace, Suit::Spades);
        assert_eq!(format!("{card}"), "Ace of Spades");
    }
}
