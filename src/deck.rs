use rand::Rng;

use crate::card::{rank::Rank, suit::Suit, Card};

mod deck_type;
use crate::deck::deck_type::DeckType;

static mut DECKTYPE: DeckType = DeckType::Standard;

/// A deck of cards, with basic functionality for shuffling and drawing from the deck.
///
/// # Examples
///
/// ```
/// use french_suited_playing_cards::{Card, Deck, Rank, Suit};
///
/// let mut deck = Deck::new();
///
/// assert_eq!(deck.draw(), Some(Card::new(Rank::King, Suit::Spades)));
/// assert_eq!(deck.draw(), Some(Card::new(Rank::Queen, Suit::Spades)));
/// assert_eq!(deck.draw(), Some(Card::new(Rank::Jack, Suit::Spades)));
/// ```
///
/// ```
/// use french_suited_playing_cards::{Card, Deck, Rank, Suit};
///
/// let mut deck = Deck::shuffled();
///
/// /* ... */
///
/// let mut hand: Vec<Card> = Vec::new();
/// while hand.len() < 5 {
///     if let Some(card) = deck.draw() {
///         hand.push(card);
///     } else {
///         // deck is empty, no card available
///         // => choose to add new shuffled deck
///         deck = Deck::shuffled();
///     }
/// }
/// assert_eq!(hand.len(), 5);
/// ```
pub struct Deck {
    decktype: DeckType,
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut deck = Deck {
            decktype: unsafe { DECKTYPE },
            cards: Vec::with_capacity(unsafe { DECKTYPE }.decksize()),
        };
        for s in Suit::iterator() {
            for r in deck.decktype.ranks() {
                deck.cards.push(Card::new(r, s));
            }
        }
        deck
    }

    pub fn shuffled() -> Deck {
        let mut deck = Deck::new();
        deck.shuffle();
        deck
    }

    pub fn size(&self) -> usize {
        self.cards.len()
    }

    pub fn draw(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    pub fn shuffle(&mut self) {
        const N: usize = 1000;
        self.shuffle_n_times(N);
    }

    pub fn shuffle_n_times(&mut self, n: usize) {
        let i: usize = 0;
        let mut j: usize;

        for _ in 0..n {
            j = rand::thread_rng().gen_range(1..self.size());
            self.cards.swap(i, j);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordered() {
        let mut deck = Deck::new();

        assert_eq!(deck.draw(), Some(Card::new(Rank::King, Suit::Spades)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Queen, Suit::Spades)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Jack, Suit::Spades)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Ten, Suit::Spades)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Nine, Suit::Spades)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Eight, Suit::Spades)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Seven, Suit::Spades)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Six, Suit::Spades)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Five, Suit::Spades)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Four, Suit::Spades)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Three, Suit::Spades)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Two, Suit::Spades)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Ace, Suit::Spades)));

        assert_eq!(deck.draw(), Some(Card::new(Rank::King, Suit::Hearts)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Queen, Suit::Hearts)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Jack, Suit::Hearts)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Ten, Suit::Hearts)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Nine, Suit::Hearts)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Eight, Suit::Hearts)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Seven, Suit::Hearts)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Six, Suit::Hearts)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Five, Suit::Hearts)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Four, Suit::Hearts)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Three, Suit::Hearts)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Two, Suit::Hearts)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Ace, Suit::Hearts)));

        assert_eq!(deck.draw(), Some(Card::new(Rank::King, Suit::Diamonds)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Queen, Suit::Diamonds)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Jack, Suit::Diamonds)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Ten, Suit::Diamonds)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Nine, Suit::Diamonds)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Eight, Suit::Diamonds)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Seven, Suit::Diamonds)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Six, Suit::Diamonds)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Five, Suit::Diamonds)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Four, Suit::Diamonds)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Three, Suit::Diamonds)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Two, Suit::Diamonds)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Ace, Suit::Diamonds)));

        assert_eq!(deck.draw(), Some(Card::new(Rank::King, Suit::Clubs)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Queen, Suit::Clubs)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Jack, Suit::Clubs)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Ten, Suit::Clubs)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Nine, Suit::Clubs)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Eight, Suit::Clubs)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Seven, Suit::Clubs)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Six, Suit::Clubs)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Five, Suit::Clubs)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Four, Suit::Clubs)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Three, Suit::Clubs)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Two, Suit::Clubs)));
        assert_eq!(deck.draw(), Some(Card::new(Rank::Ace, Suit::Clubs)));

        assert_eq!(deck.draw(), None);
    }
}
