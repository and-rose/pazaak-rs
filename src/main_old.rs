mod cards;
mod messages;
use colored::Colorize;

use std::io::Write;

fn new_game() -> cards::Game {
    let new_game = cards::Game::new();

    new_game
}

fn take_input(player: usize) -> String {
    let mut input = String::new();

    print!("P{}> ", player);
    std::io::stdout().flush().unwrap();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input
}

fn make_turn(game: &mut cards::Game) {
    let board_deck = &mut game.deck;

    for i in 0..2 {
        let player_deck = &mut game.players[i].deck;

        let drawn_card = board_deck.draw();
        player_deck.cards.push(drawn_card);

        // Await player input
        let result = take_input(i);

        println!("got -> {}", result);
    }
}

fn main() {
    messages::print_welcome_message();

    let mut game = new_game();

    println!("Board Deck has {} cards!", game.deck.cards.len());

    loop {
        // Show Game State
        for j in 0..2 {
            println!("{}", game.to_string());
            println!(
                "P{} Deck: {}",
                j,
                game.players[j].deck.to_string().italic().blue()
            );
        }
        make_turn(&mut game);
    }
}
