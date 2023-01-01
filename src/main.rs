mod cards;
mod messages;
mod util;
use cards::SpecialType;
use core::time;
use crossterm::style::Stylize;
use regex::Regex;
use std::{collections::HashSet, env, fmt, io::Write, process, thread};
use util::SPECIAL_CARD_REGEXES;

enum Action {
    Draw,
    Stand,
    EndTurn,
    Play,
    Cancel,
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
            Action::Cancel => write!(f, "Cancel"),
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
        Action::Cancel => {
            format!(
                "Cancelling {}'s current action...",
                format!("Player {}", player + 1)
            )
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

// Show iterable object with indexes
fn print_options_with_index<T>(vector: &Vec<T>)
where
    T: fmt::Display,
{
    for (i, object) in vector.iter().enumerate() {
        println!("{}: {:+}", i, object);
    }
}

fn take_card_input(player: usize, hand: &cards::Hand) -> Option<usize> {
    let mut input = String::new();

    let input_indicator = format!("(0-{}, cancel)", hand.cards.len() - 1);

    println!(
        "Which card would you like to play? {}",
        input_indicator.yellow().italic()
    );

    print_options_with_index(&hand.cards);

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
    // Display the available options in the cards values with the index beside them

    let available_options = special_card.values_list.len();
    let input_indicator = format!("(0-{}, cancel)", available_options - 1);

    println!(
        "How would you like to play this card? {}",
        input_indicator.yellow().italic()
    );
    print_options_with_index(&special_card.values_list);

    let mut input = String::new();

    print!("{}> ", player_number_to_identifier(player_number));
    std::io::stdout().flush().unwrap();

    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    input = input.trim().to_string();

    // Check if the user cancelled the input
    if input == "cancel" {
        return None;
    }

    let playstyle_index = input.parse::<usize>().unwrap();

    if playstyle_index > special_card.values_list.len() - 1 {
        print_log(messages::INVALID_INPUT_MESSAGE);
        take_playstyle_input(player_number, special_card)
    } else {
        Some(playstyle_index)
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

                match card_index {
                    Some(card_index) => {
                        let card = &mut player.hand.cards[card_index];

                        let additional_input_cards: HashSet<SpecialType> =
                            [SpecialType::Flip, SpecialType::TieBreaker]
                                .iter()
                                .cloned()
                                .collect();

                        if additional_input_cards.contains(&card.special_type) {
                            let result = take_playstyle_input(player_number, &card);

                            match result {
                                Some(result) => {
                                    card.resolve_value(result);
                                }
                                None => {
                                    return (false, false);
                                }
                            }
                        }

                        let values_list = card.values_list.clone();

                        // if the card has a board effect, apply it
                        if let Some(board_effect) = card.board_effect {
                            board_effect(player_board, values_list);
                        }

                        let mut card = player.hand.cards.remove(card_index);
                        let card_type = card.special_type.clone();

                        if player.double_next_card {
                            player.double_next_card = false;
                            card.value = card.value * 2;
                        }

                        player_board.cards.push(card);

                        if card_type == SpecialType::Double {
                            // Ask the player what card they want to double
                            print_log(messages::DOUBLE_CARD_MESSAGE);
                            player.double_next_card = true;
                            return (false, false);
                        }

                        return (false, true);
                    }
                    None => {
                        return (false, false);
                    }
                }
            } else {
                print_log(messages::ALREADY_PLAYED_MESSAGE);
                return (false, true);
            }
        }
        Action::EndTurn => {
            print_log(&get_action_message(player_number, action));
        }
        Action::Cancel => {
            print_log(&get_action_message(player_number, action));
            return (false, false);
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
                        board_effect: None,
                    });
                }
                SpecialType::Invert => {
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
                        board_effect: Some(|board, values_list| {
                            for card in &mut board.cards {
                                if values_list.contains(&card.value) {
                                    card.value *= -1;
                                }
                            }
                        }),
                    });
                }
                SpecialType::Double => {
                    // This is the only card that allows playing twice in a row
                    return Some(cards::Card {
                        values_list: vec![0],
                        value: 0,
                        special_type: *card_type,
                        board_effect: None,
                    });
                }
                SpecialType::TieBreaker => {
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
                        board_effect: None,
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
