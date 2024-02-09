mod cards;
mod messages;
mod util;

use cards::{Match, SpecialType};
use clap::Parser;
use core::time;
use crossterm::style::Stylize;
use std::{
    collections::{HashMap, HashSet},
    fs,
    io::{self, Write},
    process, thread,
};
use util::{get_action_message, print_action_log, print_log, Action};

use crate::util::print_options;

fn player_number_to_identifier(player: usize) -> &'static str {
    match player {
        0 => "You",
        1 => "Opponent",
        _ => panic!("Unexpected player number"),
    }
}

// Expecting a string of "draw", "stand", or "play" if it isn't one of those then it will return an error
fn get_input(player: usize) -> Action {
    println!(
        "What would you like to do? {}",
        "(stand, play, end)".yellow().italic()
    );

    print!("{}> ", player_number_to_identifier(player));
    io::stdout().flush().unwrap(); // Ensure the prompt appears immediately

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim() {
        "stand" => Action::Stand,
        "play" => Action::Play,
        "end" => Action::EndTurn,
        _ => {
            print_log(messages::INVALID_INPUT_MESSAGE);
            get_input(player) // Recursive call for invalid input
        }
    }
}

struct TurnResult {
    is_finished: bool,
    played_card: bool,
}

fn make_turn(pazaak_match: &mut Match) {
    for i in 0..2 {
        print_action_log(i, Action::TurnStart);

        // Skip if player is standing
        if let cards::Status::Standing = pazaak_match.players[i].status {
            print_action_log(i, Action::Stand);
            continue;
        }

        // Draw a card to the player's board from the board deck
        let drawn_card = pazaak_match.current_game().deck.draw().unwrap();
        pazaak_match.current_game().board[i].cards.push(drawn_card);

        print_action_log(i, Action::Draw);

        let mut is_finished = false;
        let mut played_card = false;

        while !is_finished {
            println!("{}", pazaak_match);

            // Get the player's input
            let action = get_input(i);
            TurnResult {
                is_finished,
                played_card,
            } = process_action(action, i, pazaak_match, played_card);
        }

        // Check if the player busted
        if pazaak_match.players[i].status == cards::Status::Busted {
            print_log(&format!(
                "{} {}",
                player_number_to_identifier(i),
                messages::BUSTED_MESSAGE
            ));
        }
    }

    // Increment the turn counter
    pazaak_match.current_game().turn += 1;
}

fn take_card_input(player: usize, hand: &cards::Hand) -> Option<usize> {
    let mut input = String::new();

    let input_indicator = format!("(0-{}, cancel)", hand.cards.len() - 1);

    println!(
        "Which card would you like to play? {}",
        input_indicator.yellow().italic()
    );

    print_options(&hand.cards);

    print!("{}> ", player_number_to_identifier(player));
    std::io::stdout().flush().unwrap();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input = input.trim().to_string();

    if input == "cancel" {
        return None;
    }

    let card_index = input.parse::<usize>().unwrap();

    if card_index > hand.cards.len() - 1 {
        print_log(messages::INVALID_INPUT_MESSAGE);
        take_card_input(player, hand)
    } else {
        Some(card_index)
    }
}

// Presents the player with the available methods of playing a card and takes their input
fn take_playstyle_input(player_number: usize, special_card: &cards::Card) -> Option<usize> {
    let values_count = special_card.values_list.len();
    if values_count == 0 {
        return None; // Early return if no options available
    }

    let input_indicator = format!("(0-{}, cancel)", values_count - 1);
    println!(
        "How would you like to play this card? {}",
        input_indicator.yellow().italic()
    );
    print_options(&special_card.values_list);

    print!("{}> ", player_number_to_identifier(player_number));
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim(); // Trim once and use this trimmed version

    if input.eq_ignore_ascii_case("cancel") {
        return None;
    }

    match input.parse::<usize>() {
        Ok(playstyle_index) if playstyle_index < values_count => Some(playstyle_index),
        _ => {
            print_log(messages::INVALID_INPUT_MESSAGE);
            take_playstyle_input(player_number, special_card) // Recursive call for invalid input
        }
    }
}

fn process_action(
    action: Action,
    player_number: usize,
    pazaak_match: &mut Match,
    already_played: bool,
) -> TurnResult {
    let player = &mut pazaak_match.players[player_number];
    let player_board =
        &mut pazaak_match.games[pazaak_match.match_detail.round - 1].board[player_number];

    match action {
        Action::Stand => {
            print_log(&get_action_message(player_number, action));
            player.status = cards::Status::Standing;
        }
        Action::Play => {
            if !already_played {
                print_log(&get_action_message(player_number, action));

                let card_index = take_card_input(player_number, &player.hand);

                match card_index {
                    Some(card_index) => {
                        let card = &mut player.hand.cards[card_index];

                        let additional_input_cards: HashSet<SpecialType> =
                            [SpecialType::Flip, SpecialType::TieBreaker]
                                .iter()
                                .cloned()
                                .collect();

                        if additional_input_cards.contains(&card.special_type) {
                            let result = take_playstyle_input(player_number, card);

                            match result {
                                Some(result) => {
                                    card.resolve_value(result);
                                }
                                None => {
                                    return TurnResult {
                                        is_finished: false,
                                        played_card: false,
                                    };
                                }
                            }
                        }

                        // if the card has a board effect, apply it
                        if let Some(board_effect) = card.board_effect {
                            board_effect(player_board, card);
                        }

                        let card = player.hand.cards.remove(card_index);

                        player_board.cards.push(card);

                        return TurnResult {
                            is_finished: false,
                            played_card: true,
                        };
                    }
                    None => {
                        return TurnResult {
                            is_finished: false,
                            played_card: false,
                        };
                    }
                }
            } else {
                print_log(messages::ALREADY_PLAYED_MESSAGE);
                return TurnResult {
                    is_finished: false,
                    played_card: false,
                };
            }
        }
        Action::EndTurn => {
            print_log(&get_action_message(player_number, action));
            if player_board.total() > 20 {
                player.status = cards::Status::Busted;
            }
        }
        _ => {
            print_log(&get_action_message(player_number, action));
        }
    }

    TurnResult {
        is_finished: true,
        played_card: false,
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

fn read_deck_file(path: &str) -> cards::Deck {
    let mut deck = cards::Deck::new();

    let mut card_counts: HashMap<SpecialType, i8> = [
        (SpecialType::None, 24),
        (SpecialType::Invert, 12),
        (SpecialType::Flip, 12),
        (SpecialType::Double, 1),
        (SpecialType::TieBreaker, 1),
    ]
    .iter()
    .cloned()
    .collect();

    let file_content = fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Unable to read file at path: {}", path));

    for line in file_content.lines() {
        // create a card based on the regex form of the card
        let card = cards::Card::from_string(line).unwrap_or_else(|| {
            eprintln!("Invalid Card in Deck: '{}'", path);
            process::exit(1);
        });

        // Increment the count of the card type
        let count = card_counts.entry(card.special_type).or_default();

        if *count == 0 {
            eprintln!("Too many cards of type: '{}'", card.special_type);
            eprintln!("Please resolve invalid Deck at Path: '{}'", path);
            process::exit(1);
        } else {
            *count -= 1;
        }

        deck.cards.push(card);
    }

    deck
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Sets the player deck file path
    #[clap(value_parser)]
    player_deck_path: String,

    /// Sets the opponent deck file path
    #[clap(value_parser)]
    opponent_deck_path: String,
}

fn main() {
    let args = Args::parse();

    let player_deck_path = args.player_deck_path;
    let opponent_deck_path = args.opponent_deck_path;

    let deck_paths = vec![player_deck_path.to_string(), opponent_deck_path.to_string()];
    validate_deck_paths(&deck_paths);
    let mut player_deck = read_deck_file(&player_deck_path);
    let mut opponent_deck = read_deck_file(&opponent_deck_path);
    // Shuffle each player's deck
    player_deck.shuffle();
    opponent_deck.shuffle();

    messages::print_welcome_message();

    let mut pzk_match = cards::Match::new(player_deck, opponent_deck);

    // Host Match
    while pzk_match.check_win().is_none() {
        pzk_match.new_game();

        // Turn Logic
        loop {
            println!("{}", "===========================".blue());
            println!("{}", pzk_match.match_detail);

            make_turn(&mut pzk_match);

            // Check if both players are standing
            if pzk_match
                .players
                .iter()
                .all(|player| player.status == cards::Status::Standing)
            {
                break;
            }

            // Check if a player busted
            if pzk_match
                .players
                .iter()
                .any(|player| player.status == cards::Status::Busted)
            {
                break;
            }
        }
        // Post Game Logic
        let winner = pzk_match.current_game().check_win();
        match winner {
            Some(winner) => {
                println!("{} wins!", player_number_to_identifier(winner));
                pzk_match.match_detail.score[winner] += 1;
            }
            None => println!("Draw!"),
        }

        // Wait 2000ms
        thread::sleep(time::Duration::from_millis(250));
    }

    // Post Match Logic
    println!("{}", "===========================".blue());
    println!("{}", pzk_match);
    let winner = pzk_match.check_win();
    match winner {
        Some(winner) => {
            println!("{} wins!", player_number_to_identifier(winner));
        }
        None => println!("Draw!"),
    }
}
