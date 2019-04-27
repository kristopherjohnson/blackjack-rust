//! Types that represent a deck of cards.
//!
//! There is nothing specific to the rules of Blackjack in this module. It can
//! be reused as-is for other card games.

extern crate rand;

use rand::seq::SliceRandom;
use rand::thread_rng;

use std::fmt;
use std::ops::Index;

/// A card's suit.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Suit {
    Clubs,
    Diamonds,
    Hearts,
    Spades,
}

/// Array of all `Suit` values.
pub const ALL_SUITS: [Suit; 4] = [Suit::Clubs, Suit::Diamonds, Suit::Hearts, Suit::Spades];

impl Suit {
    /// Returns a single-character symbol associated with the suit.
    ///
    /// Examples:
    ///
    /// ```
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
    Ace = 14,
}

/// Array of all `Rank` values
pub const ALL_RANKS: [Rank; 13] = [
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
    Rank::Ace,
];

impl Rank {
    /// Returns a single-character symbol for the rank.
    ///
    /// Examples:
    ///
    /// ```
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
    /// ```
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

/// A playing card, with a rank and suit.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Card {
    rank: Rank,
    suit: Suit,
}

impl Card {
    /// Returns the card's `Rank`
    pub fn rank(self) -> Rank {
        self.rank
    }

    /// Returns the card's `Suit`
    pub fn suit(self) -> Suit {
        self.suit
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.rank.symbol(), self.suit.symbol())
    }
}

/// Creates a `Card` with specified rank and suit.
///
/// Examples:
///
/// ```
/// use blackjack::cards::card;
/// use blackjack::cards::Rank::*;
/// use blackjack::cards::Suit::*;
///
/// let aceOfSpades = card(Ace, Spades);
/// assert_eq!(aceOfSpades.rank(), Ace);
/// assert_eq!(aceOfSpades.suit(), Spades);
///
/// let twoOfDiamonds = card(Two, Diamonds);
/// assert_eq!(twoOfDiamonds.rank(), Two);
/// assert_eq!(twoOfDiamonds.suit(), Diamonds);
/// ```
pub fn card(rank: Rank, suit: Suit) -> Card {
    Card { rank, suit }
}

/// A `Hand` is a set of cards held by a player.
#[derive(Debug)]
pub struct Hand {
    cards: Vec<Card>,
}

impl Default for Hand {
    fn default() -> Self {
        Hand { cards: vec![] }
    }
}

impl Index<usize> for Hand {
    type Output = Card;

    fn index(&self, index: usize) -> &Card {
        &self.cards[index]
    }
}

impl Hand {
    /// Returns the count of cards in the hand.
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Returns `true` if the hand contains no cards.
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Adds a card to the hand.
    pub fn push(&mut self, card: Card) {
        self.cards.push(card);
    }
}

/// A collection of cards.
///
/// The cards are ordered from the bottom to the top of the deck.  So, drawing a
/// card is taking one off the end.
#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Index<usize> for Deck {
    type Output = Card;

    fn index(&self, index: usize) -> &Card {
        &self.cards[index]
    }
}

impl Default for Deck {
    /// Returns an ordered deck of 52 cards.
    ///
    /// Examples:
    ///
    /// ```
    /// use blackjack::cards::{card, Deck};
    /// use blackjack::cards::Rank::*;
    /// use blackjack::cards::Suit::*;
    ///
    /// let deck = Deck::default();
    /// assert_eq!(deck.len(), 52);
    /// assert_eq!(deck[0], card(Two, Clubs));
    /// assert_eq!(deck[1], card(Three, Clubs));
    /// // ...
    /// assert_eq!(deck[50], card(King, Spades));
    /// assert_eq!(deck[51], card(Ace, Spades));
    /// ```
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

impl Deck {
    /// Returns a shuffled deck of 52 cards.
    ///
    /// Examples:
    ///
    /// ```
    /// use blackjack::cards::Deck;
    ///
    /// let deck = Deck::shuffled();
    /// assert_eq!(deck.len(), 52);
    /// ```
    pub fn shuffled() -> Deck {
        let mut deck = Deck::default();
        deck.shuffle();
        deck
    }

    /// Shuffles the cards.
    pub fn shuffle(&mut self) {
        self.cards.shuffle(&mut thread_rng());
    }

    /// Returns count of cards remaining in the deck.
    pub fn len(&self) -> usize {
        self.cards.len()
    }

    /// Returns `true` if the deck contains no cards.
    pub fn is_empty(&self) -> bool {
        self.cards.is_empty()
    }

    /// Removes the top card from the deck and returns it, or `None` if no cards
    /// remain.
    ///
    /// Examples:
    ///
    /// ```
    /// use blackjack::cards::{card, Deck};
    /// use blackjack::cards::Rank::*;
    /// use blackjack::cards::Suit::*;
    ///
    /// let mut deck = Deck::default();
    /// assert_eq!(deck.len(), 52);
    ///
    /// let card1 = deck.pop();
    /// assert_eq!(deck.len(), 51);
    /// assert_eq!(card1, Some(card(Ace, Spades)));
    ///
    /// let card2 = deck.pop();
    /// assert_eq!(deck.len(), 50);
    /// assert_eq!(card2, Some(card(King, Spades)));
    ///
    /// for _ in 1..=50 {
    ///     let card = deck.pop();
    ///     assert!(card.is_some());
    /// }
    ///
    /// let card = deck.pop();
    /// assert_eq!(card, None);
    /// ```
    pub fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    /// Deal the specified number of hands from a deck.
    ///
    /// Examples:
    ///
    /// ```
    /// use blackjack::cards::{card, Deck};
    /// use blackjack::cards::Rank::*;
    /// use blackjack::cards::Suit::*;
    ///
    /// let mut deck = Deck::default();
    /// let hands = deck.deal_hands(3, 2);
    ///
    /// assert_eq!(hands.len(), 3);
    /// assert_eq!(deck.len(), 46);
    ///
    /// assert_eq!(hands[0].len(), 2);
    /// assert_eq!(hands[0][0], card(Ace, Spades));
    /// assert_eq!(hands[0][1], card(Jack, Spades));
    ///
    /// assert_eq!(hands[1].len(), 2);
    /// assert_eq!(hands[1][0], card(King, Spades));
    /// assert_eq!(hands[1][1], card(Ten, Spades));
    ///
    /// assert_eq!(hands[2].len(), 2);
    /// assert_eq!(hands[2][0], card(Queen, Spades));
    /// assert_eq!(hands[2][1], card(Nine, Spades));
    /// ```
    pub fn deal_hands(&mut self, hand_count: u32, cards_per_hand: u32) -> Vec<Hand> {
        let mut hands = vec![];
        for _ in 0..hand_count {
            hands.push(Hand::default())
        }

        for _ in 0..cards_per_hand {
            for hand in hands.iter_mut() {
                hand.push(self.pop().expect("unable to draw card from deck"))
            }
        }

        hands
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
}
