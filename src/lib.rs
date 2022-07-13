//! # Crate for implementation of objects commonly used in card games.
//! Implements the Card-enum which has two possible values:
//! A standard card with a Rank and a Suit or a joker card with a Color.
//!
//! This is in turn used to implement the deck-struct.
//! Functionality for more deck-types than the standard 52-deck will be implemented in the future.

pub mod card;
pub use self::card::Card;
pub use self::card::{color::Color, rank::Rank, suit::Suit};

pub mod deck;
pub use self::deck::Deck;
