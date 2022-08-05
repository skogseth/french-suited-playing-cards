use crate::Card;
use crate::card::Rank::{self, *};
use crate::card::Suit::{self, *};
//use crate::card::Color::{self, *};
use crate::deck::DeckType;


pub fn base_deck<IterRank, IterSuit>(ranks: IterRank, suits: IterSuit) -> Vec<Card>
where
    IterRank: Iterator<Item = Rank> + Clone,
    IterSuit: Iterator<Item = Suit>,
{
    let mut cards = Vec::new();
    for s in suits {
        let ranks = ranks.clone();
        for r in ranks {
            cards.push(Card::Standard(r, s));
        }
    }
    cards
}


pub struct Standard;

impl DeckType for Standard {
    fn cards(&self) -> Vec<Card> {
        let ranks = Rank::iterator();
        let suits = Suit::iterator();
        base_deck(ranks, suits)
    }

    fn decksize(&self) -> usize { 52 }
}

pub struct Piquet;

impl DeckType for Piquet {
    fn cards(&self) -> Vec<Card> {
        let ranks = vec![Ace, Seven, Eight, Nine, Ten, Jack, Queen, King].into_iter();
        let suits = Suit::iterator();
        base_deck(ranks, suits)
    }

    fn decksize(&self) -> usize { 32 }
}


pub struct Jass;

impl DeckType for Jass {
    fn cards(&self) -> Vec<Card> {
        let ranks = vec![Ace, Six, Seven, Eight, Nine, Ten, Jack, Queen, King].into_iter();
        let suits = Suit::iterator();
        base_deck(ranks, suits)
    }

    fn decksize(&self) -> usize { 36 }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_sizes() {
        assert_eq!(Standard.cards().len(), 52);
        assert_eq!(Piquet.cards().len(), 32);
        assert_eq!(Jass.cards().len(), 36);

        assert_eq!(Standard.decksize(), 52);
        assert_eq!(Piquet.decksize(), 32);
        assert_eq!(Jass.decksize(), 36);
    }
}
