//! # Crate for implementation of objects commonly used in card games.
//! Implements the Card-struct and its two connected enumerators: Suit and Rank.
//! This is in turn used to implement the deck-struct.
//! Functionality for more deck-types than the standard 52-deck will be implemented in the future.

pub mod card;
pub use self::card::{rank::Rank, suit::Suit, Card};

pub mod deck;
pub use self::deck::Deck;
