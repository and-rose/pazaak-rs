use crossterm::style::Stylize;

// String templates for messages
const WELCOME_MESSAGE: &str = "Welcome to pazaak-rs!";
pub const INVALID_INPUT_MESSAGE: &str = "Invalid input, please try again.";
pub const INVALID_DECK_PATH_MESSAGE: &str = "Could not find deck file at path:";
pub const INVALID_DECK_FILE_MESSAGE: &str = "Invalid deck file, please try again.";
pub const INVALID_ARGUMENTS_MESSAGE: &str = "Invalid arguments, please try again.";
pub const USAGE_MESSAGE: &str = "Usage: pazaak-rs <player_deck_path> <opponent_deck_path>";
pub const ALREADY_PLAYED_MESSAGE: &str =
    "You have already played this turn, please end your turn or stand.";
pub const PREPARING_NEXT_GAME_MESSAGE: &str = "Preparing next game...";

pub fn print_welcome_message() {
    println!("{}", "===========================".blue().bold());
    println!("{}", WELCOME_MESSAGE.red().italic());
}
