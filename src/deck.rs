use rand::Rng;

pub mod decktype;

use crate::{Card, Deck};
use crate::card::{Color, Rank, Suit};

pub trait DeckType {
    fn cards(&self) -> Vec<Card>;
    fn decksize(&self) -> usize {
        self.cards().len()
    }
}

impl Deck {
    pub fn new() -> Deck {
        let decktype = Box::new(decktype::Standard);
        let cards = decktype.cards();
        Deck { decktype, cards }
    }

    pub fn shuffled() -> Deck {
        let mut deck = Deck::new();
        deck.shuffle();
        deck
    }

    pub fn empty() -> Deck {
        let decktype = Box::new(decktype::Standard);
        let cards = Vec::with_capacity(decktype.decksize());
        Deck { decktype, cards }
    }

    pub fn new_from_decktype(decktype: Box<dyn DeckType>) -> Deck {
        let cards = decktype.cards();
        Deck { decktype, cards }
    }

    pub fn shuffled_from_decktype(decktype: Box<dyn DeckType>) -> Deck {
        let mut deck = Deck::new_from_decktype(decktype);
        deck.shuffle();
        deck
    }

    pub fn empty_from_decktype(decktype: Box<dyn DeckType>) -> Deck {
        let cards = Vec::with_capacity(decktype.decksize());
        Deck { decktype, cards }
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
            j = rand::thread_rng().gen_range(1..self.cards.len());
            self.cards.swap(i, j);
        }
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

    pub fn remove_matches(&mut self, card: Card) {
        self.cards.retain(|c| *c != card);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::card::{Color::*, Rank::*, Suit::*};

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

    #[test]
    fn add_cards() {
        let mut deck = Deck::empty();

        deck.add_card(Card::Standard(Five, Clubs));
        deck.add_card(Card::Joker(Red));
        deck.add_standard(Queen, Spades);
        deck.add_joker(Black);

        assert_eq!(deck.draw(), Some(Card::Joker(Black)));
        assert_eq!(deck.draw(), Some(Card::Standard(Queen, Spades)));
        assert_eq!(deck.draw(), Some(Card::Joker(Red)));
        assert_eq!(deck.draw(), Some(Card::Standard(Five, Clubs)));
        assert_eq!(deck.draw(), None);
    }

    #[test]
    fn remove_cards() {
        let mut deck = Deck::empty();

        for r in Rank::iterator() {
            deck.add_card(Card::Standard(r, Spades));
        }

        deck.remove_matches(Card::Standard(King, Spades));
        deck.remove_matches(Card::Standard(Ten, Spades));
        deck.remove_matches(Card::Standard(Four, Spades));
        deck.remove_matches(Card::Standard(Three, Spades));

        assert_eq!(deck.draw(), Some(Card::Standard(Queen, Spades)));
        assert_eq!(deck.draw(), Some(Card::Standard(Jack, Spades)));
        assert_eq!(deck.draw(), Some(Card::Standard(Nine, Spades)));
        assert_eq!(deck.draw(), Some(Card::Standard(Eight, Spades)));
        assert_eq!(deck.draw(), Some(Card::Standard(Seven, Spades)));
        assert_eq!(deck.draw(), Some(Card::Standard(Six, Spades)));
        assert_eq!(deck.draw(), Some(Card::Standard(Five, Spades)));
        assert_eq!(deck.draw(), Some(Card::Standard(Two, Spades)));
        assert_eq!(deck.draw(), Some(Card::Standard(Ace, Spades)));
        assert_eq!(deck.draw(), None);
    }
}
