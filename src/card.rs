pub mod color;
pub mod rank;
pub mod suit;
use crate::card::{color::Color, rank::Rank, suit::Suit};

/// Either a standard card with a specified rank and suit or a joker with a specified color.
///
/// # Examples
///
/// ```
/// use french_suited_playing_cards::{Card, Rank, Suit};
///
/// let card = Card::Standard(Rank::Ace, Suit::Spades);
///
/// if let Card::Standard(rank, suit) = card {
///     assert_eq!(rank, Rank::Ace);
///     assert_eq!(suit, Suit::Spades);
/// } else {
///     panic!("rank and suit not found for card {}", card);
/// }
/// ```
///
/// ```
/// use french_suited_playing_cards::{Card, Color};
///
/// let joker = Card::Joker(Color::Red);
///
/// if let Card::Joker(color) = joker {
///     assert_eq!(color, Color::Red);
/// } else {
///     panic!("color not found for card {}", joker);
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Card {
    Standard(Rank, Suit),
    Joker(Color),
}

impl Card {
    /// # Examples
    ///
    /// ```
    /// use french_suited_playing_cards::{Card, Rank, Suit};
    ///
    /// let card = Card::Standard(Rank::Eight, Suit::Diamonds);
    ///
    /// let rank = card.rank(); //will panic if joker
    /// assert_eq!(rank, Rank::Eight);
    /// ```
    pub fn rank(&self) -> Rank {
        if let Card::Standard(rank, _) = self {
            *rank
        } else {
            panic!("rank not found for card: {}", self);
        }
    }

    /// # Examples
    ///
    /// ```
    /// use french_suited_playing_cards::{Card, Rank, Suit};
    ///
    /// let card = Card::Standard(Rank::Jack, Suit::Clubs);
    ///
    /// let suit = card.suit(); //will panic if joker
    /// assert_eq!(suit, Suit::Clubs);
    /// ```
    pub fn suit(&self) -> Suit {
        if let Card::Standard(_, suit) = self {
            *suit
        } else {
            panic!("suit not found for card: {}", self);
        }
    }

    /// # Examples
    ///
    /// ```
    /// use french_suited_playing_cards::{Card, Rank, Suit};
    ///
    /// let card = Card::Standard(Rank::King, Suit::Hearts);
    ///
    /// let (rank, suit) = card.rank_and_suit(); //will panic if joker
    /// assert_eq!(rank, Rank::King);
    /// assert_eq!(suit, Suit::Hearts);
    /// ```
    pub fn rank_and_suit(&self) -> (Rank, Suit) {
        if let Card::Standard(rank, suit) = self {
            (*rank, *suit)
        } else {
            panic!("rank & suit not found for card: {}", self);
        }
    }

    /// # Examples
    ///
    /// ```
    /// use french_suited_playing_cards::{Card, Color};
    ///
    /// let card = Card::Joker(Color::Black);
    ///
    /// let color = card.color(); //will panic if standard card
    /// assert_eq!(color, Color::Black);
    /// ```
    pub fn color(&self) -> Color {
        if let Card::Joker(color) = self {
            *color
        } else {
            panic!("color not found for card: {}", self);
        }
    }
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Card::Standard(rank, suit) => write!(f, "{} of {}", rank, suit),
            Card::Joker(color) => write!(f, "{} joker", color),
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
                card = Card::Standard(r, s);
                println!("{}", card);
            }
        }
        for c in Color::iterator() {
            card = Card::Joker(c);
            println!("{}", card);
        }
    }

    #[test]
    fn display() {
        let card = Card::Standard(Rank::Ace, Suit::Spades);
        assert_eq!(format!("{card}"), "Ace of Spades");

        let joker = Card::Joker(Color::Red);
        assert_eq!(format!("{joker}"), "Red joker")
    }
}
