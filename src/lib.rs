//! # Crate for implementation of objects commonly used in card games.
//! Implements the Card-enum which has two possible values:
//! A standard card with a Rank and a Suit or a joker card with a Color.
//!
//! This is in turn used to implement the deck-struct.
//! Functionality for more deck-types is a work in progress,
//! but currently supports Standard (with/without jokers), Piquet and Jass decks,
//! as well as the oppurtunity to create own types using the DeckType trait.

pub mod card;
pub mod deck;

use crate::card::{Color, Rank, Suit};
use crate::deck::DeckType;

/// Either a standard card with a specified rank and suit or a joker with a specified color.
///
/// # Examples
///
/// ```
/// use french_suited_playing_cards::{Card, card::Rank, card::Suit};
///
/// let card = Card::Standard(Rank::Ace, Suit::Spades);
///
/// if let Card::Standard(rank, suit) = card {
///     assert_eq!(rank, Rank::Ace);
///     assert_eq!(suit, Suit::Spades);
/// } else {
///     panic!("rank and suit not found for card {}", card);
/// }
/// ```
///
/// ```
/// use french_suited_playing_cards::{Card, card::Color};
///
/// let joker = Card::Joker(Color::Red);
///
/// if let Card::Joker(color) = joker {
///     assert_eq!(color, Color::Red);
/// } else {
///     panic!("color not found for card {}", joker);
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Card {
    Standard(Rank, Suit),
    Joker(Color),
}

/// A deck of cards, with basic functionality for shuffling and drawing from the deck.
///
/// # Examples
///
/// ```
/// use french_suited_playing_cards::{Card, Deck, card::Rank, card::Suit};
///
/// let mut deck = Deck::new();
///
/// assert_eq!(deck.draw(), Some(Card::Standard(Rank::King, Suit::Spades)));
/// assert_eq!(deck.draw(), Some(Card::Standard(Rank::Queen, Suit::Spades)));
/// assert_eq!(deck.draw(), Some(Card::Standard(Rank::Jack, Suit::Spades)));
/// ```
///
/// ```
/// use french_suited_playing_cards::{Card, Deck, card::Rank, card::Suit};
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
///         // => choose to replace with new shuffled deck
///         deck = Deck::shuffled();
///     }
/// }
/// assert_eq!(hand.len(), 5);
/// ```
pub struct Deck {
    decktype: Box<dyn DeckType>,
    cards: Vec<Card>,
}