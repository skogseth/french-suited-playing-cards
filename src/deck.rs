use rand::Rng;

use crate::card::{suit::Suit, Card};

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
/// assert_eq!(deck.draw(), Some(Card::standard(Rank::King, Suit::Spades)));
/// assert_eq!(deck.draw(), Some(Card::standard(Rank::Queen, Suit::Spades)));
/// assert_eq!(deck.draw(), Some(Card::standard(Rank::Jack, Suit::Spades)));
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
                deck.cards.push(Card::standard(r, s));
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
                deck.cards.push(Card::standard(r, s));
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

    pub fn remove_card(&self, card: Card) {}
    pub fn add_card(&self) {}
    pub fn add_jokers(&self, n: usize) {}
    pub fn add_deck(&self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::rank::Rank;

    #[test]
    fn ordered() {
        let mut deck = Deck::new();

        assert_eq!(deck.draw(), Some(Card::standard(Rank::King, Suit::Spades)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Queen, Suit::Spades)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Jack, Suit::Spades)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Ten, Suit::Spades)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Nine, Suit::Spades)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Eight, Suit::Spades)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Seven, Suit::Spades)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Six, Suit::Spades)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Five, Suit::Spades)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Four, Suit::Spades)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Three, Suit::Spades)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Two, Suit::Spades)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Ace, Suit::Spades)));

        assert_eq!(deck.draw(), Some(Card::standard(Rank::King, Suit::Hearts)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Queen, Suit::Hearts)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Jack, Suit::Hearts)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Ten, Suit::Hearts)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Nine, Suit::Hearts)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Eight, Suit::Hearts)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Seven, Suit::Hearts)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Six, Suit::Hearts)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Five, Suit::Hearts)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Four, Suit::Hearts)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Three, Suit::Hearts)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Two, Suit::Hearts)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Ace, Suit::Hearts)));

        assert_eq!(
            deck.draw(),
            Some(Card::standard(Rank::King, Suit::Diamonds))
        );
        assert_eq!(
            deck.draw(),
            Some(Card::standard(Rank::Queen, Suit::Diamonds))
        );
        assert_eq!(
            deck.draw(),
            Some(Card::standard(Rank::Jack, Suit::Diamonds))
        );
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Ten, Suit::Diamonds)));
        assert_eq!(
            deck.draw(),
            Some(Card::standard(Rank::Nine, Suit::Diamonds))
        );
        assert_eq!(
            deck.draw(),
            Some(Card::standard(Rank::Eight, Suit::Diamonds))
        );
        assert_eq!(
            deck.draw(),
            Some(Card::standard(Rank::Seven, Suit::Diamonds))
        );
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Six, Suit::Diamonds)));
        assert_eq!(
            deck.draw(),
            Some(Card::standard(Rank::Five, Suit::Diamonds))
        );
        assert_eq!(
            deck.draw(),
            Some(Card::standard(Rank::Four, Suit::Diamonds))
        );
        assert_eq!(
            deck.draw(),
            Some(Card::standard(Rank::Three, Suit::Diamonds))
        );
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Two, Suit::Diamonds)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Ace, Suit::Diamonds)));

        assert_eq!(deck.draw(), Some(Card::standard(Rank::King, Suit::Clubs)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Queen, Suit::Clubs)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Jack, Suit::Clubs)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Ten, Suit::Clubs)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Nine, Suit::Clubs)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Eight, Suit::Clubs)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Seven, Suit::Clubs)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Six, Suit::Clubs)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Five, Suit::Clubs)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Four, Suit::Clubs)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Three, Suit::Clubs)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Two, Suit::Clubs)));
        assert_eq!(deck.draw(), Some(Card::standard(Rank::Ace, Suit::Clubs)));

        assert_eq!(deck.draw(), None);
    }
}
