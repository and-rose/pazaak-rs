use crate::cards::SpecialType;

// Regex for a card with a value
pub const CARD_REGEX: &str = r"^([+-]?\d+)$";

// Regex for a Tiebreaker card which usually looks like "+1/-1T"
pub const TIEBREAKER_REGEX: &str = r"^([+-]?\d+)/([+-]?\d)+T$";

// Regex for a Flip card which usually looks like "+1/-1" capture the postive and negative values
pub const FLIP_REGEX: &str = r"^([+-]?\d+)/([+-]?\d)$";

// Regex for a Swap card which usually looks like "2&4" capture the two values
pub const SWAP_REGEX: &str = r"^(\d+)&(\d+)$";

// Regex for a Double card which usually looks like "D" capture the D
pub const DOUBLE_REGEX: &str = r"^D$";

// Hashmap of all the special card types and their regex
pub const SPECIAL_CARD_REGEXES: &[(SpecialType, &str)] = &[
    (SpecialType::TieBreaker, TIEBREAKER_REGEX),
    (SpecialType::Flip, FLIP_REGEX),
    (SpecialType::Swap, SWAP_REGEX),
    (SpecialType::Double, DOUBLE_REGEX),
    (SpecialType::None, CARD_REGEX),
];
