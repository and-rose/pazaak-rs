mod ai;
mod cards;
mod messages;
mod util;

use cards::{Match, SpecialType};
use core::time;
use crossterm::style::Stylize;
use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    env,
    io::Write,
    process, thread,
};
use util::{get_action_message, print_action_log, print_log, Action, SPECIAL_CARD_REGEXES};

use crate::util::print_options_with_index;

fn player_number_to_identifier(player: usize) -> String {
    match player {
        0 => String::from("You"),
        1 => String::from("Opponent"),
        _ => format!("Player {}", player + 1),
    }
}

// Expecting a string of "draw", "stand", or "play" if it isn't one of those then it will return an error
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

    //TODO: concat the play steps into 1 command
    let dummy_card = cards::Card::new(-1);
    match input.as_str() {
        "stand" => Action::Stand,
        "play" => Action::Play(dummy_card),
        "end" => Action::EndTurn,
        _ => {
            print_log(messages::INVALID_INPUT_MESSAGE);
            get_input(player)
        }
    }
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

        let best_move = ai::get_best_move(pazaak_match, 100);
        match best_move {
            Some(best_move) => {
                println!("Best move: {:?}", best_move);
            }
            None => {
                println!("No best move found");
            }
        }

        while !is_finished {
            println!("{}", pazaak_match);

            // Get the player's input
            let action = get_input(i);
            (is_finished, played_card) = process_action(action, i, pazaak_match, played_card);
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
    pazaak_match: &mut Match,
    already_played: bool,
) -> (bool, bool) {
    let player = &mut pazaak_match.players[player_number];
    let player_board =
        &mut pazaak_match.games[pazaak_match.match_detail.round - 1].board[player_number];

    match action {
        Action::Stand => {
            print_log(&get_action_message(player_number, action));
            player.status = cards::Status::Standing;
        }
        Action::Play(_) => {
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

                        // if the card has a board effect, apply it
                        if let Some(board_effect) = card.board_effect {
                            board_effect(player_board, card);
                        }

                        let card = player.hand.cards.remove(card_index);

                        player_board.cards.push(card);

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
            if player_board.total() > 20 {
                player.status = cards::Status::Busted;
            }
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
                        board_effect: Some(|board, played_card| {
                            for card in &mut board.cards {
                                if played_card.values_list.contains(&card.value) {
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
                        board_effect: Some(|board, played_card| {
                            // Find out what the last card played was on the board
                            match board.cards.last() {
                                Some(last_card) => {
                                    // If the last card played was a double, play the card again
                                    played_card.value = last_card.value;
                                }
                                None => {}
                            }
                        }),
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

    let mut card_counts: HashMap<SpecialType, i8> = HashMap::new();
    card_counts.insert(SpecialType::None, 24);
    card_counts.insert(SpecialType::Invert, 12);
    card_counts.insert(SpecialType::Flip, 12);
    card_counts.insert(SpecialType::Double, 1);
    card_counts.insert(SpecialType::TieBreaker, 1);

    let file = std::fs::read_to_string(path).expect("Unable to read file");

    for line in file.lines() {
        // create a card based on the regex form of the card
        let card = create_card_from_string(line)
            .expect(&format!("Unable to create card from string for {}", line));

        // Increment the count of the card type
        let count = card_counts.entry(card.special_type).or_insert(0);

        if count == &0 {
            eprintln!("{} '{}'", "Too many cards of type:", card.special_type);
            eprintln!("{} '{}'", "Please resolve invalid Deck at Path:", path);
            process::exit(1);
        }
        *count -= 1;

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
    let mut player_deck = read_deck_file(player_deck_path);
    let mut opponent_deck = read_deck_file(opponent_deck_path);
    // Shuffle each player's deck
    player_deck.shuffle();
    opponent_deck.shuffle();

    messages::print_welcome_message();

    let mut pzk_match = cards::Match::new(player_deck, opponent_deck);

    // Host Match
    while pzk_match.match_detail.score[0] < 3 && pzk_match.match_detail.score[1] < 3 {
        pzk_match.new_game();

        // Turn Logic
        loop {
            println!("{}", "===========================".blue());
            println!("{}", pzk_match.match_detail);

            make_turn(&mut pzk_match); // Player turn
                                       // make_opponent_turn(); // Opponent turn

            // Check if both players are standing
            if pzk_match.players[0].status == cards::Status::Standing
                && pzk_match.players[1].status == cards::Status::Standing
            {
                break;
            }

            // Check if a player busted
            if pzk_match.players[0].status == cards::Status::Busted
                || pzk_match.players[1].status == cards::Status::Busted
            {
                break;
            }
        }
        // Post Game Logic
        let winner = pzk_match.check_win();
        match winner {
            Some(winner) => {
                println!("{} wins!", player_number_to_identifier(winner));
                pzk_match.match_detail.score[winner] = pzk_match.match_detail.score[winner] + 1;
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
