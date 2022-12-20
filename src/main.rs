mod cards;

fn new_game() -> cards::Game {
    let new_game = cards::Game::new();

    new_game
}

fn main() {
    let game = new_game();

    println!("Board Deck has {} cards!", game.deck.cards.len());
}
