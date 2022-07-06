#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rank {
    Ace = 1,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
}

use self::Rank::*;

impl Rank {
    fn to_str(&self) -> &str {
        match self {
            Ace => "Ace",
            Two => "Two",
            Three => "Three",
            Four => "Four",
            Five => "Five",
            Six => "Six",
            Seven => "Seven",
            Eight => "Eight",
            Nine => "Nine",
            Ten => "Ten",
            Jack => "Jack",
            Queen => "Queen",
            King => "King",
        }
    }

    pub fn iterator() -> impl Iterator<Item = Rank> {
        [
            Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King,
        ]
        .iter()
        .copied()
    }
}

impl std::fmt::Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterate() {
        let mut iter = Rank::iterator();
        assert_eq!(iter.next(), Some(Ace));
        assert_eq!(iter.next(), Some(Two));
        assert_eq!(iter.next(), Some(Three));
        assert_eq!(iter.next(), Some(Four));
        assert_eq!(iter.next(), Some(Five));
        assert_eq!(iter.next(), Some(Six));
        assert_eq!(iter.next(), Some(Seven));
        assert_eq!(iter.next(), Some(Eight));
        assert_eq!(iter.next(), Some(Nine));
        assert_eq!(iter.next(), Some(Ten));
        assert_eq!(iter.next(), Some(Jack));
        assert_eq!(iter.next(), Some(Queen));
        assert_eq!(iter.next(), Some(King));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn print() {
        for r in Rank::iterator() {
            println!("Rank: {}", r);
        }
    }

    #[test]
    fn values() {
        for (i, r) in Rank::iterator().enumerate() {
            assert_eq!(i + 1 as usize, r as usize);
        }
    }
}
