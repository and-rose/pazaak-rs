use crossterm::style::Stylize;
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
                self.cards.push(Card { value: i + 1 });
            }
        }
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut deck_string = String::new();

        if &self.cards.len() == &0 {
            return write!(f, "{}", "<Empty Deck>".to_string().yellow().italic());
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

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut hand_string = String::new();

        if &self.cards.len() == &0 {
            return write!(f, "{}", "<Empty Hand>".to_string().yellow().italic());
        }

        for i in 0..self.cards.len() {
            hand_string.push_str(&self.cards[i].value.to_string());

            if i != self.cards.len() - 1 {
                hand_string.push_str(", ");
            }
        }

        write!(f, "{}", hand_string)
    }
}

pub struct Player {
    pub hand: Hand,
    pub deck: Deck,
}

pub struct Board {
    pub cards: Vec<Card>,
}

impl Board {
    pub fn total(&self) -> u8 {
        let mut total = 0;

        for i in 0..self.cards.len() {
            total += self.cards[i].value;
        }

        total
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut board_string = String::new();

        if &self.cards.len() == &0 {
            return write!(f, "{}", "<Empty Board>".to_string().yellow().italic());
        }

        for i in 0..self.cards.len() {
            board_string.push_str(&self.cards[i].value.to_string());

            if i != self.cards.len() - 1 {
                board_string.push_str(", ");
            }
        }

        write!(f, "{} ({})", board_string, self.total())
    }
}
// A Game is a collection of players, boards, and a deck
pub struct Game {
    pub players: [Player; 2],
    pub board: [Board; 2],
    pub score: [u8; 2],
    pub deck: Deck,
    pub turn: u8,
    pub round: u8,
    pub winner: u8,
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
            score: [0, 0],
            turn: 1,
            round: 1,
            winner: 0,
        }
    }
}

// Show Game State

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut game_string = String::new();

        let game_details = format!("Round: {} | Turn: {}\n", self.round, self.turn);

        game_string.push_str(&format!("{}", game_details.blue()));

        let player1_details = format!("You: {}", self.score[0]);
        let player2_details = format!("Opponent: {}", self.score[1]);

        game_string.push_str(&format!("{}", player1_details.green(),));
        game_string.push_str(&format!("{}", "   | ".to_string().blue()));
        game_string.push_str(&format!("{}", player2_details.red()));

        write!(f, "{}", game_string)
    }
}
