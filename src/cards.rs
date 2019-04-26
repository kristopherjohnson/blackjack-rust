extern crate rand;

use rand::seq::SliceRandom;
use rand::thread_rng;

use std::fmt;

/// A card's suit.
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

/// Array of all `Suit` values.
pub const ALL_SUITS: [Suit; 4] = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];

impl Suit {
    /// Return symbol associated with the suit.
    ///
    /// Examples:
    ///
    /// ```rust
    /// use blackjack::cards::Suit;
    ///
    /// assert_eq!(Suit::Clubs.symbol(), "♣");
    /// assert_eq!(Suit::Diamonds.symbol(), "♦");
    /// assert_eq!(Suit::Hearts.symbol(), "♥");
    /// assert_eq!(Suit::Spades.symbol(), "♠");
    /// ```
    pub fn symbol(self) -> &'static str {
        match self {
            Suit::Clubs => "\u{2663}",
            Suit::Diamonds => "\u{2666}",
            Suit::Hearts => "\u{2665}",
            Suit::Spades => "\u{2660}",
        }
    }
}

impl fmt::Display for Suit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.symbol())
    }
}

/// A card's rank.
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub enum Rank {
    Ace = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}

/// Array of all `Rank` values
pub const ALL_RANKS: [Rank; 13] = [
    Rank::Ace,
    Rank::Two,
    Rank::Three,
    Rank::Four,
    Rank::Five,
    Rank::Six,
    Rank::Seven,
    Rank::Eight,
    Rank::Nine,
    Rank::Ten,
    Rank::Jack,
    Rank::Queen,
    Rank::King,
];

impl Rank {
    /// Return a single-character symbol for the rank.
    ///
    /// Examples:
    ///
    /// ```rust
    /// use blackjack::cards::Rank;
    ///
    /// assert_eq!(Rank::Ace.symbol(), "A");
    /// assert_eq!(Rank::Two.symbol(), "2");
    /// // ...
    /// assert_eq!(Rank::Nine.symbol(), "9");
    /// assert_eq!(Rank::Ten.symbol(), "T");
    /// assert_eq!(Rank::Jack.symbol(), "J");
    /// assert_eq!(Rank::Queen.symbol(), "Q");
    /// assert_eq!(Rank::King.symbol(), "K");
    pub fn symbol(self) -> &'static str {
        match self {
            Rank::Ace => "A",
            Rank::Two => "2",
            Rank::Three => "3",
            Rank::Four => "4",
            Rank::Five => "5",
            Rank::Six => "6",
            Rank::Seven => "7",
            Rank::Eight => "8",
            Rank::Nine => "9",
            Rank::Ten => "T",
            Rank::Jack => "J",
            Rank::Queen => "Q",
            Rank::King => "K",
        }
    }
}

/// A card.
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

/// Create a `Card`
pub fn card(rank: Rank, suit: Suit) -> Card {
    Card { rank, suit }
}

/// A collection of cards.
///
/// The cards are ordered from the bottom to the top of the deck.  So, drawing a
/// card is taking one off the end.
#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    /// Return a shuffled deck of 52 cards.
    pub fn shuffled() -> Deck {
        let mut deck = Deck::default();
        deck.shuffle();
        deck
    }

    /// Shuffle the cards.
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    /// Return a read-only reference to the cards vector.
    pub fn cards(&self) -> &[Card] {
        &self.cards
    }
}

impl Default for Deck {
    /// Return an ordered deck of 52 cards.
    fn default() -> Self {
        let mut cards = vec![];
        for &suit in ALL_SUITS.iter() {
            for &rank in ALL_RANKS.iter() {
                cards.push(card(rank, suit));
            }
        }
        Deck { cards }
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank.symbol(), self.suit.symbol())
    }
}

#[cfg(test)]
mod tests {
    use super::Rank::*;
    use super::Suit::*;
    use super::*;

    #[test]
    fn card_format() {
        assert_eq!(format!("{}", card(Ace, Clubs)), "A♣");
        assert_eq!(format!("{}", card(Two, Hearts)), "2♥");
        assert_eq!(format!("{}", card(Three, Diamonds)), "3♦");
        assert_eq!(format!("{}", card(Nine, Spades)), "9♠");
        assert_eq!(format!("{}", card(Ten, Clubs)), "T♣");
        assert_eq!(format!("{}", card(Jack, Hearts)), "J♥");
        assert_eq!(format!("{}", card(Queen, Diamonds)), "Q♦");
        assert_eq!(format!("{}", card(King, Spades)), "K♠");

        assert_eq!(
            format!("{:?}", card(Ace, Spades)),
            "Card { rank: Ace, suit: Spades }"
        );
    }

    #[test]
    fn deck_default() {
        let d = Deck::default();
        assert_eq!(d.cards.len(), 52);
        assert_eq!(d.cards[0], card(Ace, Clubs));
        assert_eq!(d.cards[1], card(Two, Clubs));
        assert_eq!(d.cards[50], card(Queen, Spades));
        assert_eq!(d.cards[51], card(King, Spades));
    }

    #[test]
    fn deck_shuffled() {
        let d = Deck::shuffled();
        assert_eq!(d.cards.len(), 52);
    }
}
