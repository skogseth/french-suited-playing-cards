#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

use self::Suit::*;

impl Suit {
    fn to_str(&self) -> &str {
        match self {
            Clubs => "Clubs",
            Diamonds => "Diamonds",
            Hearts => "Hearts",
            Spades => "Spades",
        }
    }

    pub fn iterator() -> impl Iterator<Item = Suit> {
        [Clubs, Diamonds, Hearts, Spades].iter().copied()
    }
}

impl std::fmt::Display for Suit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterate() {
        let mut iter = Suit::iterator();
        assert_eq!(iter.next(), Some(Clubs));
        assert_eq!(iter.next(), Some(Diamonds));
        assert_eq!(iter.next(), Some(Hearts));
        assert_eq!(iter.next(), Some(Spades));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn print() {
        for s in Suit::iterator() {
            println!("Suit: {}", s);
        }
    }
}
