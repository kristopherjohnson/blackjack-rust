extern crate blackjack;

use blackjack::cards::Deck;

fn main() {
    // Doesn't play a game yet.
    // Just prints out a shuffled deck.
    print!("Shuffled deck: ");
    let mut deck = Deck::shuffled();
    while let Some(card) = deck.pop() {
        print!("{} ", card)
    }
    println!();
}
