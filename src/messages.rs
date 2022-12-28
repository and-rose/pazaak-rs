use crossterm::style::Stylize;

// String templates for messages
const WELCOME_MESSAGE: &str = "Welcome to pazaak-rs!";
pub const INVALID_INPUT_MESSAGE: &str = "Invalid input, please try again.";
pub const INVALID_DECK_PATH_MESSAGE: &str = "Could not find deck file at path:";

pub fn print_welcome_message() {
    println!("{}", "===========================".blue().bold());
    println!("{}", WELCOME_MESSAGE.red().italic());
    println!("{}", "===========================".blue().bold());
}
