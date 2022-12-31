mod cards;
mod messages;
mod util;
use cards::{Board, SpecialType};
use core::time;
use crossterm::style::Stylize;
use regex::Regex;
use std::{env, fmt, io::Write, process, thread};
use util::SPECIAL_CARD_REGEXES;

enum Action {
    Draw,
    Stand,
    EndTurn,
    Play,
    TurnStart,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::Draw => write!(f, "Draw"),
            Action::Stand => write!(f, "Stand"),
            Action::Play => write!(f, "Play"),
            Action::TurnStart => write!(f, "Turn Start"),
            Action::EndTurn => write!(f, "End Turn"),
        }
    }
}

fn print_log(message: &str) {
    println!("{} {}", "~".dark_grey(), message.dark_grey());
    thread::sleep(time::Duration::from_millis(150));
}

fn print_action_log(player: usize, action: Action) {
    let message = get_action_message(player, action);
    print_log(&message);
    thread::sleep(time::Duration::from_millis(250));
}

fn get_action_message(player: usize, action: Action) -> String {
    let message = match action {
        Action::Draw => {
            format!("{} Draws...", format!("Player {}", player + 1))
        }
        Action::Stand => {
            format!("{} Stands...", format!("Player {}", player + 1))
        }
        Action::Play => {
            format!("{} Plays...", format!("Player {}", player + 1))
        }
        Action::TurnStart => {
            format!("Starting {}'s Turn...", format!("Player {}", player + 1))
        }
        Action::EndTurn => {
            format!("Ending {}'s Turn...", format!("Player {}", player + 1))
        }
    };

    message
}

fn player_number_to_identifier(player: usize) -> String {
    match player {
        0 => String::from("You"),
        1 => String::from("Opponent"),
        _ => format!("Player {}", player + 1),
    }
}

// Expecting a string of "draw", "Stand", or "play" if it isn't one of those then it will return an error
fn get_input(player: usize) -> Action {
    let mut input = String::new();

    let input_indicator = format!("{}", "(stand, play, end)");

    println!(
        "What would you like to do? {}",
        input_indicator.yellow().italic()
    );

    print!("{}> ", player_number_to_identifier(player));
    std::io::stdout().flush().unwrap();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input = input.trim().to_string();

    match input.as_str() {
        "stand" => Action::Stand,
        "play" => Action::Play,
        "end" => Action::EndTurn,
        _ => {
            print_log(messages::INVALID_INPUT_MESSAGE);
            get_input(player)
        }
    }
}

fn print_board(players: &[cards::Player; 2], board: &[cards::Board; 2]) {
    // Show Board State
    println!("{}", "---------------------------".blue().bold());

    println!("Opponent Board: {}", board[1]);
    println!(
        "Opponent Hand: {}",
        players[1].hand.get_anonymous_hand_string()
    );

    println!("{}", "~~~~~~~~~~~~~~~~~~~~~~~~~~~".blue().bold());

    println!("Your Board: {}", board[0]);
    println!("Your Hand: {}", players[0].hand);

    println!("{}", "---------------------------".blue().bold());
}

fn make_turn(game: &mut cards::Game) {
    for i in 0..2 {
        print_action_log(i, Action::TurnStart);

        // Skip turn if player is standing
        if let cards::Status::Standing = game.players[i].status {
            print_action_log(i, Action::Stand);
            continue;
        }

        let board_deck = &mut game.deck;
        let drawn_card = board_deck.draw().expect(messages::DECK_EMPTY);
        let player_board = &mut game.board[i];
        player_board.cards.push(drawn_card);

        print_action_log(i, Action::Draw);

        let mut is_finished = false;
        let mut played_card = false;

        while !is_finished {
            print_board(&game.players, &game.board);
            // Await player input
            let result = get_input(i);
            (is_finished, played_card) = process_action(
                result,
                i,
                &mut game.players[i],
                &mut game.board[i],
                played_card,
            );
        }
    }
    game.turn = game.turn + 1;
}

// Show hand with indexes beside them
fn print_hand_with_indexes(hand: &cards::Hand) {
    for (i, card) in hand.cards.iter().enumerate() {
        println!("{}: {}", i, card);
    }
}

fn take_card_input(player: usize, hand: &cards::Hand) -> usize {
    let mut input = String::new();

    let input_indicator = format!("(0-{})", hand.cards.len() - 1);

    println!(
        "Which card would you like to play? {}",
        input_indicator.yellow().italic()
    );

    print_hand_with_indexes(hand);

    print!("{}> ", player_number_to_identifier(player));
    std::io::stdout().flush().unwrap();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input = input.trim().to_string();

    let card_index = input.parse::<usize>().unwrap();

    if card_index > hand.cards.len() - 1 {
        print_log(messages::INVALID_INPUT_MESSAGE);
        take_card_input(player, hand)
    } else {
        card_index
    }
}

fn process_action(
    action: Action,
    player_number: usize,
    player: &mut cards::Player,
    player_board: &mut cards::Board,
    already_played: bool,
) -> (bool, bool) {
    match action {
        Action::Stand => {
            print_log(&get_action_message(player_number, action));
            player.status = cards::Status::Standing;
        }
        Action::Play => {
            if !already_played {
                print_log(&get_action_message(player_number, action));

                let card_index = take_card_input(player_number, &player.hand);

                let card = player.hand.cards.remove(card_index);

                player_board.cards.push(card);

                return (false, true);
            } else {
                print_log(messages::ALREADY_PLAYED_MESSAGE);
                return (false, true);
            }
        }
        Action::EndTurn => {
            print_log(&get_action_message(player_number, action));
        }
        _ => {
            print_log(&get_action_message(player_number, action));
        }
    }

    (true, false)
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

fn create_card_from_string(card_string: &str) -> Option<cards::Card> {
    // Check each regex for a match
    // If a match is found, create a card based on the regex
    // If no match is found, return an error
    // If multiple matches are found, return an error

    for (card_type, regex) in SPECIAL_CARD_REGEXES.iter() {
        // println!("{:?}: {}", card_type, regex);
        let regex_string = Regex::new(regex).unwrap_or_else(|_| {
            eprintln!("{} '{}'", "fail", regex);
            process::exit(1);
        });
        if regex_string.is_match(card_string) {
            // Create a card based on the regex
            match card_type {
                SpecialType::None => {
                    // Create a card based on the regex
                    let value = regex_string
                        .captures(card_string)
                        .unwrap()
                        .get(1)
                        .unwrap()
                        .as_str()
                        .parse::<i8>()
                        .unwrap();

                    return Some(cards::Card::new(value));
                }
                SpecialType::Flip => {
                    // Create a card based on the two groups in the first match
                    let captures = regex_string.captures(card_string).unwrap();
                    let values: Vec<i8> = captures
                        .iter()
                        .skip(1)
                        .map(|x| x.unwrap().as_str().parse::<i8>().unwrap())
                        .collect();

                    return Some(cards::Card {
                        values_list: values,
                        value: 0,
                        special_type: *card_type,
                        board_effect: Some(|board| {}),
                    });
                }
                SpecialType::Swap => {
                    // Create a card based on the regex
                    let captures = regex_string.captures(card_string).unwrap();
                    let values: Vec<i8> = captures
                        .iter()
                        .skip(1)
                        .map(|x| x.unwrap().as_str().parse::<i8>().unwrap())
                        .collect();

                    return Some(cards::Card {
                        values_list: values,
                        value: 0,
                        special_type: *card_type,
                        board_effect: Some(|board| {}),
                    });
                }
                SpecialType::Double => {
                    // Create a card based on the regex

                    return Some(cards::Card {
                        values_list: vec![0],
                        value: 0,
                        special_type: *card_type,
                        board_effect: Some(|board| {}),
                    });
                }
                SpecialType::TieBreaker => {
                    // Create a card based on the regex
                    let captures = regex_string.captures(card_string).unwrap();
                    let values: Vec<i8> = captures
                        .iter()
                        .skip(1)
                        .map(|x| x.unwrap().as_str().parse::<i8>().unwrap())
                        .collect();

                    return Some(cards::Card {
                        values_list: values,
                        value: 0,
                        special_type: *card_type,
                        board_effect: Some(|board| {}),
                    });
                }
            }
        }
    }

    println!("No match found for '{}'", card_string);

    None
}

fn read_deck_file(path: &str) -> cards::Deck {
    let mut deck = cards::Deck::new();

    let file = std::fs::read_to_string(path).expect("Unable to read file");

    // Various card types

    for line in file.lines() {
        // create a card based on the regex form of the card
        let card = create_card_from_string(line)
            .expect(&format!("Unable to create card from string for {}", line));
        deck.cards.push(card);
    }

    deck
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!(
            "{} {}",
            messages::INVALID_ARGUMENTS_MESSAGE,
            messages::USAGE_MESSAGE
        );
        process::exit(1);
    }
    let player_deck_path = &args[1];
    let opponent_deck_path = &args[2];
    let deck_paths = vec![player_deck_path.to_string(), opponent_deck_path.to_string()];
    validate_deck_paths(&deck_paths);

    messages::print_welcome_message();

    let mut pzk_match = cards::Match::new();

    // Host Match
    while pzk_match.score[0] < 3 && pzk_match.score[1] < 3 {
        let player_deck = read_deck_file(player_deck_path);
        let opponent_deck = read_deck_file(opponent_deck_path);
        pzk_match.new_game(player_deck, opponent_deck);

        // Turn Logic
        loop {
            println!("{}", "===========================".blue());
            println!("{}", pzk_match);
            let current_game = pzk_match.current_game();
            make_turn(current_game);
            if current_game.players[0].status == cards::Status::Standing
                && current_game.players[1].status == cards::Status::Standing
            {
                break;
            }
        }
        // Post Game Logic
        let winner = pzk_match.games[pzk_match.round - 1].check_win();
        match winner {
            Some(winner) => {
                println!("{} wins!", player_number_to_identifier(winner));
                pzk_match.score[winner] = pzk_match.score[winner] + 1;
            }
            None => println!("Draw!"),
        }

        // Wait 2000ms
        thread::sleep(time::Duration::from_millis(750));
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
