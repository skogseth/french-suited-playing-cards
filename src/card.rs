pub mod color;
pub mod rank;
pub mod suit;
use crate::card::{color::Color, rank::Rank, suit::Suit};

/// A card with a specified rank and suit.
///
/// # Examples
///
/// ```
/// use french_suited_playing_cards::{Card, Rank, Suit};
///
/// let card = Card::new(Rank::Ace, Suit::Spades);
/// assert_eq!(card.rank(), Rank::Ace);
/// assert_eq!(card.suit(), Suit::Spades);
/// assert_eq!(format!("{card}"), "Ace of Spades");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
    card: CardType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CardType {
    Standard(Rank, Suit),
    Joker(Color),
}

impl Card {
    pub fn standard(rank: Rank, suit: Suit) -> Card {
        Card {
            card: CardType::Standard(rank, suit),
        }
    }

    pub fn joker(color: Color) -> Card {
        Card {
            card: CardType::Joker(color),
        }
    }

    pub fn rank(&self) -> Option<Rank> {
        if let CardType::Standard(rank, _) = self.card {
            Some(rank)
        } else {
            None
        }
    }

    pub fn suit(&self) -> Option<Suit> {
        if let CardType::Standard(_, suit) = self.card {
            Some(suit)
        } else {
            None
        }
    }

    pub fn color(&self) -> Option<Color> {
        if let CardType::Joker(color) = self.card {
            Some(color)
        } else {
            None
        }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.card {
            CardType::Standard(rank, suit) => write!(f, "{} of {}", rank, suit),
            CardType::Joker(color) => write!(f, "{} joker", color),
        }
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
                card = Card::standard(r, s);
                println!("{}", card);
            }
        }
    }

    #[test]
    fn display() {
        let card = Card::standard(Rank::Ace, Suit::Spades);
        assert_eq!(format!("{card}"), "Ace of Spades");
    }
}
