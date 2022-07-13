use rand::Rng;

use crate::card::{color::Color, rank::Rank, suit::Suit, Card};

mod deck_type;
use crate::deck::deck_type::DeckType;

/// A deck of cards, with basic functionality for shuffling and drawing from the deck.
///
/// # Examples
///
/// ```
/// use french_suited_playing_cards::{Card, Deck, Rank, Suit};
///
/// let mut deck = Deck::new();
///
/// assert_eq!(deck.draw(), Some(Card::Standard(Rank::King, Suit::Spades)));
/// assert_eq!(deck.draw(), Some(Card::Standard(Rank::Queen, Suit::Spades)));
/// assert_eq!(deck.draw(), Some(Card::Standard(Rank::Jack, Suit::Spades)));
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
        let decktype = DeckType::Standard;
        let cards = Vec::with_capacity(decktype.decksize());

        let mut deck = Deck { decktype, cards };

        for s in Suit::iterator() {
            for r in deck.decktype.ranks() {
                deck.cards.push(Card::Standard(r, s));
            }
        }

        deck
    }

    pub fn shuffled() -> Deck {
        let mut deck = Deck::new();
        deck.shuffle();
        deck
    }

    pub fn new_given(decktype: DeckType) -> Deck {
        let cards = Vec::with_capacity(decktype.decksize());

        let mut deck = Deck { decktype, cards };

        for s in Suit::iterator() {
            for r in deck.decktype.ranks() {
                deck.cards.push(Card::Standard(r, s));
            }
        }

        deck
    }

    pub fn shuffled_given(decktype: DeckType) -> Deck {
        let mut deck = Deck::new_given(decktype);
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

    pub fn remove_match(&mut self, card: Card) {
        self.cards.retain(|c| *c != card);
    }

    pub fn add_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn add_standard(&mut self, rank: Rank, suit: Suit) {
        self.cards.push(Card::Standard(rank, suit));
    }

    pub fn add_joker(&mut self, color: Color) {
        self.cards.push(Card::Joker(color));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{rank::Rank::*, suit::Suit::*};

    #[test]
    fn ordered() {
        let mut deck = Deck::new();

        assert_eq!(deck.draw(), Some(Card::Standard(King, Spades)));
        assert_eq!(deck.draw(), Some(Card::Standard(Queen, Spades)));
        assert_eq!(deck.draw(), Some(Card::Standard(Jack, Spades)));
        assert_eq!(deck.draw(), Some(Card::Standard(Ten, Spades)));
        assert_eq!(deck.draw(), Some(Card::Standard(Nine, Spades)));
        assert_eq!(deck.draw(), Some(Card::Standard(Eight, Spades)));
        assert_eq!(deck.draw(), Some(Card::Standard(Seven, Spades)));
        assert_eq!(deck.draw(), Some(Card::Standard(Six, Spades)));
        assert_eq!(deck.draw(), Some(Card::Standard(Five, Spades)));
        assert_eq!(deck.draw(), Some(Card::Standard(Four, Spades)));
        assert_eq!(deck.draw(), Some(Card::Standard(Three, Spades)));
        assert_eq!(deck.draw(), Some(Card::Standard(Two, Spades)));
        assert_eq!(deck.draw(), Some(Card::Standard(Ace, Spades)));

        assert_eq!(deck.draw(), Some(Card::Standard(King, Hearts)));
        assert_eq!(deck.draw(), Some(Card::Standard(Queen, Hearts)));
        assert_eq!(deck.draw(), Some(Card::Standard(Jack, Hearts)));
        assert_eq!(deck.draw(), Some(Card::Standard(Ten, Hearts)));
        assert_eq!(deck.draw(), Some(Card::Standard(Nine, Hearts)));
        assert_eq!(deck.draw(), Some(Card::Standard(Eight, Hearts)));
        assert_eq!(deck.draw(), Some(Card::Standard(Seven, Hearts)));
        assert_eq!(deck.draw(), Some(Card::Standard(Six, Hearts)));
        assert_eq!(deck.draw(), Some(Card::Standard(Five, Hearts)));
        assert_eq!(deck.draw(), Some(Card::Standard(Four, Hearts)));
        assert_eq!(deck.draw(), Some(Card::Standard(Three, Hearts)));
        assert_eq!(deck.draw(), Some(Card::Standard(Two, Hearts)));
        assert_eq!(deck.draw(), Some(Card::Standard(Ace, Hearts)));

        assert_eq!(deck.draw(), Some(Card::Standard(King, Diamonds)));
        assert_eq!(deck.draw(), Some(Card::Standard(Queen, Diamonds)));
        assert_eq!(deck.draw(), Some(Card::Standard(Jack, Diamonds)));
        assert_eq!(deck.draw(), Some(Card::Standard(Ten, Diamonds)));
        assert_eq!(deck.draw(), Some(Card::Standard(Nine, Diamonds)));
        assert_eq!(deck.draw(), Some(Card::Standard(Eight, Diamonds)));
        assert_eq!(deck.draw(), Some(Card::Standard(Seven, Diamonds)));
        assert_eq!(deck.draw(), Some(Card::Standard(Six, Diamonds)));
        assert_eq!(deck.draw(), Some(Card::Standard(Five, Diamonds)));
        assert_eq!(deck.draw(), Some(Card::Standard(Four, Diamonds)));
        assert_eq!(deck.draw(), Some(Card::Standard(Three, Diamonds)));
        assert_eq!(deck.draw(), Some(Card::Standard(Two, Diamonds)));
        assert_eq!(deck.draw(), Some(Card::Standard(Ace, Diamonds)));

        assert_eq!(deck.draw(), Some(Card::Standard(King, Clubs)));
        assert_eq!(deck.draw(), Some(Card::Standard(Queen, Clubs)));
        assert_eq!(deck.draw(), Some(Card::Standard(Jack, Clubs)));
        assert_eq!(deck.draw(), Some(Card::Standard(Ten, Clubs)));
        assert_eq!(deck.draw(), Some(Card::Standard(Nine, Clubs)));
        assert_eq!(deck.draw(), Some(Card::Standard(Eight, Clubs)));
        assert_eq!(deck.draw(), Some(Card::Standard(Seven, Clubs)));
        assert_eq!(deck.draw(), Some(Card::Standard(Six, Clubs)));
        assert_eq!(deck.draw(), Some(Card::Standard(Five, Clubs)));
        assert_eq!(deck.draw(), Some(Card::Standard(Four, Clubs)));
        assert_eq!(deck.draw(), Some(Card::Standard(Three, Clubs)));
        assert_eq!(deck.draw(), Some(Card::Standard(Two, Clubs)));
        assert_eq!(deck.draw(), Some(Card::Standard(Ace, Clubs)));

        assert_eq!(deck.draw(), None);
    }
}
