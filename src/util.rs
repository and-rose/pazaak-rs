use core::time;
use crossterm::style::Stylize;
use std::{fmt, thread};

use crate::cards::SpecialType;

// Regex for a card with a value
pub const CARD_REGEX: &str = r"^([+-]?\d+)$";

// Regex for a Tiebreaker card which usually looks like "+1/-1T"
// These cards will win the game in the case of a tie
pub const TIEBREAKER_REGEX: &str = r"^([+-]?\d+)/([+-]?\d)+T$";

// Regex for a Flip card which usually looks like "+1/-1" capture the postive and negative values
// The cards can be flipped before they are played.
pub const FLIP_REGEX: &str = r"^([+-]?\d+)/([+-]?\d)$";

// Regex for a Swap card which usually looks like "2&4" capture the two values
// The cards swap the values on the board corresponding to the values on the card.
pub const SWAP_REGEX: &str = r"^(\d+)&(\d+)$";

// Regex for a Double card which usually looks like "D" capture the D
// These cards double the value of the board
pub const DOUBLE_REGEX: &str = r"^D$";

// Hashmap of all the special card types and their regex
pub const SPECIAL_CARD_REGEXES: &[(SpecialType, &str)] = &[
    (SpecialType::TieBreaker, TIEBREAKER_REGEX),
    (SpecialType::Flip, FLIP_REGEX),
    (SpecialType::Invert, SWAP_REGEX),
    (SpecialType::Double, DOUBLE_REGEX),
    (SpecialType::None, CARD_REGEX),
];

pub enum Action {
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

pub fn get_action_message(player: usize, action: Action) -> String {
    let player_str = format!("Player {}", player + 1);
    match action {
        Action::Draw => format!("{} Draws...", player_str),
        Action::Stand => format!("{} Stands...", player_str),
        Action::Play => format!("{} Plays...", player_str),
        Action::TurnStart => format!("Starting {}'s Turn...", player_str),
        Action::EndTurn => format!("Ending {}'s Turn...", player_str),
    }
}

pub fn print_log(message: &str) {
    println!("{} {}", "~".dark_grey(), message.dark_grey());
    thread::sleep(time::Duration::from_millis(150));
}

pub fn print_action_log(player: usize, action: Action) {
    let message = get_action_message(player, action);
    print_log(&message);
    thread::sleep(time::Duration::from_millis(250));
}

// Show iterable object with indexes
pub fn print_options<T>(vector: &[T])
where
    T: fmt::Display,
{
    vector.iter().enumerate().for_each(|(i, object)| {
        println!("{}: {}", i + 1, object);
    });
}
