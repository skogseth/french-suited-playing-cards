use rand::Rng;

use crate::card::{rank::Rank, suit::Suit, Card};

pub const DEFAULT_DECKSIZE: usize = 52;

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut deck = Deck {
            cards: Vec::with_capacity(DEFAULT_DECKSIZE),
        };
        for r in Rank::iterator() {
            for s in Suit::iterator() {
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
