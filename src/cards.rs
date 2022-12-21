use rand::seq::SliceRandom;
use std::fmt;

pub struct Card {
    pub value: u8,
}

pub struct SpecialCard {
    pub value: u8,
    pub effect: fn(),
    pub effect_test: String,
}

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        Deck { cards: vec![] }
    }

    pub fn shuffle(&mut self) {
        // Shuffle the deck
        self.cards.shuffle(&mut rand::thread_rng());
    }

    pub fn draw(&mut self) -> Card {
        // Draw Card
        self.cards.pop().unwrap()
    }

    pub fn default_fill(&mut self) {
        for _ in 0..4 {
            for i in 0..10 {
                self.cards.push(Card { value: i });
            }
        }
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut deck_string = String::new();

        if &self.cards.len() == &0 {
            return write!(f, "<Empty Deck>");
        }

        for i in 0..self.cards.len() {
            deck_string.push_str(&self.cards[i].value.to_string());

            if i != self.cards.len() - 1 {
                deck_string.push_str(", ");
            }
        }

        write!(f, "{}", deck_string)
    }
}

pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand { cards: vec![] }
    }
}

pub struct Player {
    pub hand: Hand,
    pub deck: Deck,
}

pub struct Board {
    pub cards: Vec<Card>,
}

pub struct Game {
    pub players: [Player; 2],
    pub board: [Board; 2],
    pub deck: Deck,
}

impl Game {
    pub fn new() -> Game {
        let player1 = Player {
            hand: Hand::new(),
            deck: Deck::new(),
        };

        let player2 = Player {
            hand: Hand::new(),
            deck: Deck::new(),
        };

        let board1 = Board { cards: vec![] };
        let board2 = Board { cards: vec![] };

        // Generate Game Deck
        let mut board_deck = Deck::new();
        board_deck.default_fill();
        board_deck.shuffle();

        Game {
            players: [player1, player2],
            board: [board1, board2],
            deck: board_deck,
        }
    }
}
