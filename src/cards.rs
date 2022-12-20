pub struct Card {
    value: u8,
}

pub struct SpecialCard {
    value: u8,
    effect: fn(),
    effect_test: String,
}

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        Deck { cards: vec![] }
    }
}

pub struct Hand {
    cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand { cards: vec![] }
    }
}

pub struct Player {
    hand: Hand,
    deck: Deck,
}

pub struct Board {
    cards: Vec<Card>,
}

pub struct Game {
    players: [Player; 2],
    board: [Board; 2],
}
