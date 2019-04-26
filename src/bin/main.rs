extern crate blackjack;

use blackjack::cards::Deck;

fn main() {
    // Doesn't play a game yet.
    // Just prints out a shuffled deck.
    println!("Shuffled deck:");
    let deck = Deck::shuffled();
    for card in deck.cards() {
        print!("{} ", card)
    }
    println!();
}
