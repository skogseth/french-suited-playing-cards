#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
    Red,
    Black,
    White,
}

use self::Color::*;

impl Color {
    fn to_str(&self) -> &str {
        match self {
            Red => "Red",
            Black => "Black",
            White => "White",
        }
    }

    pub fn iterator() -> impl Iterator<Item = Color> {
        [Red, Black, White].iter().copied()
    }
}

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterate() {
        let mut iter = Color::iterator();
        assert_eq!(iter.next(), Some(Red));
        assert_eq!(iter.next(), Some(Black));
        assert_eq!(iter.next(), Some(White));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn print() {
        for c in Color::iterator() {
            println!("Color: {}", c);
        }
    }
}
