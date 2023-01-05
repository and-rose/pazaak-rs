use core::time;
use crossterm::style::Stylize;
use std::{fmt, thread};

use crate::cards::{self, Card, SpecialType};

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
    Play(Card),
    Cancel,
    TurnStart,
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Action::Draw => write!(f, "Draw"),
            Action::Stand => write!(f, "Stand"),
            Action::Play(_) => write!(f, "Play"),
            Action::TurnStart => write!(f, "Turn Start"),
            Action::EndTurn => write!(f, "End Turn"),
            Action::Cancel => write!(f, "Cancel"),
        }
    }
}

pub fn get_action_message(player: usize, action: Action) -> String {
    let card = cards::Card::new(0);

    let message = match action {
        Action::Draw => {
            format!("{} Draws...", format!("Player {}", player + 1))
        }
        Action::Stand => {
            format!("{} Stands...", format!("Player {}", player + 1))
        }
        Action::Play(_) => {
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
pub fn print_options_with_index<T>(vector: &Vec<T>)
where
    T: fmt::Display,
{
    for (i, object) in vector.iter().enumerate() {
        println!("{}: {:+}", i, object);
    }
}
