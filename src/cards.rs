use crossterm::style::Stylize;
use rand::seq::SliceRandom;
use std::fmt;

use crate::messages;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum SpecialType {
    None,
    Flip,
    Invert,
    Double,
    TieBreaker,
}

#[derive(Clone)]
pub struct Card {
    pub values_list: Vec<i8>,
    pub value: i8,
    pub special_type: SpecialType,
    pub board_effect: Option<fn(&mut Board, Vec<i8>)>,
}

impl Card {
    pub fn new(value: i8) -> Card {
        Card {
            values_list: vec![value],
            value,
            special_type: SpecialType::None,
            board_effect: None,
        }
    }

    pub fn resolve_value(&mut self, index: usize) {
        self.value = self.values_list[index];
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut card_string = String::new();

        match self.special_type {
            SpecialType::None => {
                card_string.push_str(&self.value.to_string());

                if &self.value > &0 {
                    card_string = card_string.green().to_string();
                } else if &self.value < &0 {
                    card_string = card_string.red().to_string();
                }
            }
            SpecialType::Flip => {
                // put parentheses around the list item that matches the value
                for (i, v) in self.values_list.iter().enumerate() {
                    if *v == self.value {
                        card_string.push_str(&format!("[{:+}]", v));
                    } else {
                        card_string.push_str(&format!("{:+}", v));
                    }

                    if i != self.values_list.len() - 1 {
                        card_string.push_str("/");
                    }
                }

                card_string = card_string.blue().to_string();
            }
            SpecialType::Invert => {
                card_string.push_str(&format!("{}&{}", self.values_list[0], self.values_list[1]));

                card_string = card_string.yellow().to_string();
            }
            SpecialType::Double => {
                card_string.push_str("D");

                card_string = card_string.yellow().to_string();
            }
            SpecialType::TieBreaker => {
                // put parentheses around the list item that matches the value
                for (i, v) in self.values_list.iter().enumerate() {
                    if *v == self.value {
                        card_string.push_str(&format!("[{:+}]", v));
                    } else {
                        card_string.push_str(&format!("{:+}", v));
                    }

                    if i != self.values_list.len() - 1 {
                        card_string.push_str("/");
                    }
                }

                card_string.push_str("T");

                card_string = card_string.blue().to_string();
            }
        }

        // Colouring

        write!(f, "{}", card_string)
    }
}

#[derive(Clone)]
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

    pub fn draw(&mut self) -> Option<Card> {
        // Draw Card from deck and return none if deck is empty
        self.cards.pop()
    }

    pub fn default_fill(&mut self) {
        for _ in 0..4 {
            for i in 0..10 {
                self.cards.push(Card {
                    values_list: vec![i + 1],
                    value: i + 1,
                    special_type: SpecialType::None,
                    board_effect: None,
                });
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
            deck_string.push_str(&self.cards[i].to_string());

            if i != self.cards.len() - 1 {
                deck_string.push_str(", ");
            }
        }

        write!(f, "{}", deck_string)
    }
}

#[derive(Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
}

impl Hand {
    pub fn new() -> Hand {
        Hand { cards: vec![] }
    }

    // Gets a string of the hand but with the values hidden by question marks
    pub fn get_anonymous_hand_string(&self) -> String {
        let mut hand_string = String::new();

        if &self.cards.len() == &0 {
            return "<Empty Hand>".yellow().italic().to_string();
        }

        for i in 0..self.cards.len() {
            hand_string.push_str("?");

            if i != self.cards.len() - 1 {
                hand_string.push_str(", ");
            }
        }

        hand_string
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut hand_string = String::new();

        if &self.cards.len() == &0 {
            return write!(f, "{}", "<Empty Hand>".to_string().yellow().italic());
        }

        for i in 0..self.cards.len() {
            hand_string.push_str(&self.cards[i].to_string());

            if i != self.cards.len() - 1 {
                hand_string.push_str(", ");
            }
        }

        write!(f, "{}", hand_string)
    }
}

#[derive(PartialEq, Clone)]
pub enum Status {
    Playing,
    Standing,
    Busted,
}

#[derive(Clone)]
pub struct Player {
    pub hand: Hand,
    pub deck: Deck,
    pub status: Status,
    pub double_next_card: bool,
}

#[derive(Clone)]
pub struct Board {
    pub cards: Vec<Card>,
}

impl Board {
    pub fn total(&self) -> i8 {
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
            board_string.push_str(&self.cards[i].to_string());

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
    pub deck: Deck,
    pub turn: u8,
    pub winner: u8,
}

impl Game {
    pub fn new(deck1: Deck, deck2: Deck) -> Game {
        let player1 = Player {
            hand: Hand::new(),
            deck: deck1,
            status: Status::Playing,
            double_next_card: false,
        };

        let player2 = Player {
            hand: Hand::new(),
            deck: deck2,
            status: Status::Playing,
            double_next_card: false,
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
            turn: 1,
            winner: 0,
        }
    }

    // Check which player won the game by comparing the total of their boards and seeing who didn't bust
    pub fn check_win(&mut self) -> Option<usize> {
        let player1_total = self.board[0].total();
        let player2_total = self.board[1].total();

        let player1_distance = 20 - player1_total;
        let player2_distance = 20 - player2_total;

        if player1_distance < 0 && player2_distance < 0 {
            // Both players busted
            return None;
        } else if player1_distance < 0 {
            // Player 1 busted
            return Some(1);
        } else if player2_distance < 0 {
            // Player 2 busted
            return Some(0);
        } else if player1_distance < player2_distance {
            // Player 1 is closer to 20
            return Some(0);
        } else if player2_distance < player1_distance {
            // Player 2 is closer to 20
            return Some(1);
        } else {
            // Both players are equally close to 20
            return None;
        }
    }
}

// has a vector of mutably borrowed games
pub struct Match {
    pub games: Vec<Game>,
    pub round: usize,
    pub score: [u8; 2],
}

impl Match {
    pub fn new() -> Match {
        Match {
            games: vec![],
            round: 0,
            score: [0, 0],
        }
    }

    pub fn new_game(&mut self, player_deck: Deck, opponent_deck: Deck) {
        let mut new_game = Game::new(player_deck, opponent_deck);

        // Shuffle the decks
        new_game.players[0].deck.shuffle();
        new_game.players[1].deck.shuffle();

        // Both players draw 5 cards from their player specific deck
        for _ in 0..5 {
            new_game.players[0]
                .hand
                .cards
                .push(new_game.players[0].deck.draw().expect(messages::DECK_EMPTY));
            new_game.players[1]
                .hand
                .cards
                .push(new_game.players[1].deck.draw().expect(messages::DECK_EMPTY));
        }

        // add the game to the match
        self.games.push(new_game);

        // Increment the round
        self.round += 1;
    }

    pub fn current_game(&mut self) -> &mut Game {
        &mut self.games[self.round - 1]
    }

    // Check which player won the match by reaching 3 points
    pub fn check_win(&mut self) -> Option<usize> {
        if self.score[0] == 3 {
            return Some(0);
        } else if self.score[1] == 3 {
            return Some(1);
        } else {
            return None;
        }
    }
}

impl fmt::Display for Match {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut match_string = String::new();

        let match_details = format!(
            "Round: {} | Turn: {}\n",
            self.round,
            self.games[self.round - 1].turn
        );

        match_string.push_str(&format!("{}", match_details.blue()));

        let player1_details = format!("You: {}", self.score[0]);
        let player2_details = format!("Opponent: {}", self.score[1]);

        match_string.push_str(&format!("{}", player1_details.green(),));
        match_string.push_str(&format!("{}", "   | ".to_string().blue()));
        match_string.push_str(&format!("{}", player2_details.red()));

        write!(f, "{}", match_string)
    }
}
