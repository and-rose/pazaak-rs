mod cards;
mod messages;
use crossterm::style::Stylize;
use std::{env, fmt, io::Write, process};

fn new_game() -> cards::Game {
    let mut new_game = cards::Game::new();

    // Both players draw 5 cards
    for _ in 0..5 {
        new_game.players[0].hand.cards.push(new_game.deck.draw());
        new_game.players[1].hand.cards.push(new_game.deck.draw());
    }

    new_game
}

enum Action {
    Draw,
    Stay,
    Play,
    TurnStart,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::Draw => write!(f, "Draw"),
            Action::Stay => write!(f, "Stay"),
            Action::Play => write!(f, "Play"),
            Action::TurnStart => write!(f, "Turn Start"),
        }
    }
}

fn print_log(message: &str) {
    println!("{} {}", "~".dark_grey(), message.dark_grey());
}

fn get_action_message(player: usize, action: Action) -> String {
    let message = match action {
        Action::Draw => {
            format!("{} Draws...", format!("Player {}", player + 1))
        }
        Action::Stay => {
            format!("{} Stays...", format!("Player {}", player + 1))
        }
        Action::Play => {
            format!("{} Plays...", format!("Player {}", player + 1))
        }
        Action::TurnStart => {
            format!("Starting {}'s Turn...", format!("Player {}", player + 1))
        }
    };

    message
}

fn print_action_log(player: usize, action: Action) {
    let message = get_action_message(player, action);
    print_log(&message);
}

// Expecting a string of "draw", "stay", or "play" if it isn't one of those then it will return an error
fn get_input(player: usize) -> Action {
    let mut input = String::new();

    match player {
        0 => print!("You> "),
        1 => print!("Opponent> "),
        _ => print!("Player {}> ", player + 1),
    }

    std::io::stdout().flush().unwrap();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input = input.trim().to_string();

    match input.as_str() {
        "draw" => Action::Draw,
        "stay" => Action::Stay,
        "play" => Action::Play,
        _ => {
            print_log(messages::INVALID_INPUT_MESSAGE);
            get_input(player)
        }
    }
}

fn print_board(players: &[cards::Player; 2], board: &[cards::Board; 2]) {
    // Show Board State
    println!("{}", "---------------------------".blue().bold());

    println!(
        "Opponent Hand: {}",
        players[1].hand.get_anonymous_hand_string()
    );
    println!("Opponent Board: {}", board[1]);

    println!("{}", "~~~~~~~~~~~~~~~~~~~~~~~~~~~".blue().bold());

    println!("Your Hand: {}", players[0].hand);
    println!("Your Board: {}", board[0]);

    println!("{}", "---------------------------".blue().bold());
}

fn make_turn(game: &mut cards::Game) {
    for i in 0..2 {
        print_board(&game.players, &game.board);
        print_action_log(i, Action::TurnStart);

        let player_deck = &mut game.players[i].deck;

        let board_deck = &mut game.deck;
        let drawn_card = board_deck.draw();

        player_deck.cards.push(drawn_card);

        // Await player input
        let result = get_input(i);

        process_action(result, i, board_deck, player_deck, &mut game.board[i]);
    }
    game.turn = game.turn + 1;
}

fn read_deck_file(path: &str) -> cards::Deck {
    let mut deck = cards::Deck::new();

    let file = std::fs::read_to_string(path).expect("Unable to read file");

    for line in file.lines() {
        let found_int: u8 = u8::from_str_radix(line, 10).expect("Unable to parse value");

        let card = cards::Card::new(found_int);
        deck.cards.push(card);
    }

    deck
}

fn process_action(
    action: Action,
    player: usize,
    board_deck: &mut cards::Deck,
    player_deck: &mut cards::Deck,
    player_board: &mut cards::Board,
) {
    match action {
        Action::Draw => {
            print_log(&get_action_message(player, action));

            let drawn_card = board_deck.draw();
            player_board.cards.push(drawn_card);
        }
        Action::Stay => {
            print_log(&get_action_message(player, action));
        }
        Action::Play => {
            print_log(&get_action_message(player, action));

            let mut input = String::new();

            println!("What card would you like to play?");

            print!("Player {}> ", player + 1);
            std::io::stdout().flush().unwrap();

            std::io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");

            input = input.trim().to_string();

            let card_index = input.parse::<usize>().unwrap();

            let card = player_deck.cards.remove(card_index - 1);

            player_board.cards.push(card);
        }
        _ => {
            print_log(&get_action_message(player, action));
        }
    }
}

fn validate_deck_paths(paths: &[String]) {
    print_log("Validating Deck Paths...");
    for path in paths {
        if !std::path::Path::new(path).exists() {
            eprintln!("{} '{}'", messages::INVALID_DECK_PATH_MESSAGE, path);
            process::exit(1);
        }
        print_log(&format!("{} '{}'", "Found Deck Path:", path));
    }
    print_log("Deck Paths Validated!");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let player_deck_path = &args[1];
    let opponent_deck_path = &args[2];
    let deck_paths = vec![player_deck_path.to_string(), opponent_deck_path.to_string()];
    validate_deck_paths(&deck_paths);

    messages::print_welcome_message();

    let mut game = new_game();

    loop {
        println!("{}", game);
        make_turn(&mut game);
    }
}
