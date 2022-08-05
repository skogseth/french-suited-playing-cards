use crate::Card;
use crate::card::Rank::{self, *};
use crate::card::Suit::{self, *};
use crate::card::Color::{self, *};
use crate::deck::DeckType;


const STANDARD_DECKSIZE: usize = 52;
const PIQUET_DECKSIZE: usize = 32;
const JASS_DECKSIZE: usize = 36;


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
    fn cards(&self) -> Vec<Card> { base_deck(Rank::iterator(), Suit::iterator()) }
    fn decksize(&self) -> usize { STANDARD_DECKSIZE }
}

pub struct Piquet;

impl DeckType for Piquet {
    fn cards(&self) -> Vec<Card> {
        let ranks = [Ace, Seven, Eight, Nine, Ten, Jack, Queen, King].into_iter();
        let suits = Suit::iterator();
        base_deck(ranks, suits)
    }

    fn decksize(&self) -> usize { PIQUET_DECKSIZE }
}

pub struct Jass;

impl DeckType for Jass {
    fn cards(&self) -> Vec<Card> {
        let ranks = [Ace, Six, Seven, Eight, Nine, Ten, Jack, Queen, King].into_iter();
        let suits = Suit::iterator();
        base_deck(ranks, suits)
    }

    fn decksize(&self) -> usize { JASS_DECKSIZE }
}

pub struct StandardWithJokers {
    colors: Vec<Color>,
    decksize: usize,
}

impl StandardWithJokers {
    fn new() -> StandardWithJokers {
        StandardWithJokers::from_colors(vec![Red, Black])
    }

    fn from_colors(colors: Vec<Color>) -> StandardWithJokers {
        let decksize =  STANDARD_DECKSIZE + colors.len();
        StandardWithJokers { colors, decksize }
    }
}

impl DeckType for StandardWithJokers {
    fn cards(&self) -> Vec<Card> {
        let mut deck = base_deck(Rank::iterator(), Suit::iterator());
        for color in self.colors.clone() {
            deck.push(Card::Joker(color));
        }  
        deck
    }

    fn decksize(&self) -> usize { self.decksize }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_decks() {
        assert_eq!(Standard.cards().len(), STANDARD_DECKSIZE);
        assert_eq!(Standard.decksize(), STANDARD_DECKSIZE);

        assert_eq!(Piquet.cards().len(), PIQUET_DECKSIZE);
        assert_eq!(Piquet.decksize(), PIQUET_DECKSIZE);

        assert_eq!(Jass.cards().len(), JASS_DECKSIZE);
        assert_eq!(Jass.decksize(), JASS_DECKSIZE);
    }

    #[test]
    fn standard() {
        let standard_with_two_jokers = StandardWithJokers::new();
        assert_eq!(standard_with_two_jokers.cards().len(), STANDARD_DECKSIZE + 2);
        assert_eq!(standard_with_two_jokers.decksize(), STANDARD_DECKSIZE + 2);

        let standard_with_three_jokers = StandardWithJokers::from_colors(vec![Red, Black, White]);
        assert_eq!(standard_with_three_jokers.cards().len(), STANDARD_DECKSIZE + 3);
        assert_eq!(standard_with_three_jokers.decksize(), STANDARD_DECKSIZE + 3);
    }
}
