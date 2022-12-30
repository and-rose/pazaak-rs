use regex::Regex;

// Regex used to identify cards in a deck file

// Regex for a card with a value
pub const CARD_REGEX: &str = r"^\d+$";

// Regex for a Tiebreaker card which usually looks like "+1/-1T"
pub const TIEBREAKER_REGEX: &str = r"^(\+?\d+)\/(-?\d)+T$";

// Regex for a Flip card which usually looks like "+1/-1" capture the postive and negative values
pub const FLIP_REGEX: &str = r"^(\+?\d+)\/(-?\d)+$";

// Regex for a Swap card which usually looks like "2&4" capture the two values
pub const SWAP_REGEX: &str = r"^(\d+)(&)(\d+)$";

// Regex for a Double card which usually looks like "D" capture the D
pub const DOUBLE_REGEX: &str = r"^D$";
