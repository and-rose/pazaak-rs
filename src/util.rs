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
