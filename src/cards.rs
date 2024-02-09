use crossterm::style::Stylize;
use rand::seq::SliceRandom;
use regex::Regex;
use std::fmt;

use crate::util::SPECIAL_CARD_REGEXES;

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum SpecialType {
    None,
    Flip,
    Invert,
    Double,
    TieBreaker,
}

impl fmt::Display for SpecialType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SpecialType::None => write!(f, "None"),
            SpecialType::Flip => write!(f, "Flip"),
            SpecialType::Invert => write!(f, "Invert"),
            SpecialType::Double => write!(f, "Double"),
            SpecialType::TieBreaker => write!(f, "TieBreaker"),
        }
    }
}

#[derive(Clone)]
pub struct Card {
    pub values_list: Vec<i8>,
    pub value: i8,
    pub special_type: SpecialType,
    pub board_effect: Option<fn(&mut Board, &mut Card)>,
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

    pub fn from_string(card_string: &str) -> Option<Card> {
        // Check each regex for a match
        // If a match is found, create a card based on the regex
        // If no match is found, return an error
        // If multiple matches are found, return an error

        for (card_type, regex) in SPECIAL_CARD_REGEXES.iter() {
            // println!("{:?}: {}", card_type, regex);
            let regex_string = Regex::new(regex).unwrap_or_else(|_| {
                panic!("Invalid regex: {}", regex);
            });

            if regex_string.is_match(card_string) {
                // Create a card based on the regex
                match card_type {
                    SpecialType::None => {
                        // Create a card based on the regex
                        let value = regex_string
                            .captures(card_string)
                            .unwrap()
                            .get(1)
                            .unwrap()
                            .as_str()
                            .parse::<i8>()
                            .unwrap();

                        return Some(Card::new(value));
                    }
                    SpecialType::Flip => {
                        // Create a card based on the two groups in the first match
                        let captures = regex_string.captures(card_string).unwrap();
                        let values: Vec<i8> = captures
                            .iter()
                            .skip(1)
                            .map(|x| x.unwrap().as_str().parse::<i8>().unwrap())
                            .collect();

                        return Some(Card {
                            values_list: values,
                            value: 0,
                            special_type: *card_type,
                            board_effect: None,
                        });
                    }
                    SpecialType::Invert => {
                        // Create a card based on the regex
                        let captures = regex_string.captures(card_string).unwrap();
                        let values: Vec<i8> = captures
                            .iter()
                            .skip(1)
                            .map(|x| x.unwrap().as_str().parse::<i8>().unwrap())
                            .collect();

                        return Some(Card {
                            values_list: values,
                            value: 0,
                            special_type: *card_type,
                            board_effect: Some(|board, played_card| {
                                for card in &mut board.cards {
                                    if played_card.values_list.contains(&card.value) {
                                        card.value *= -1;
                                    }
                                }
                            }),
                        });
                    }
                    SpecialType::Double => {
                        // This is the only card that allows playing twice in a row
                        return Some(Card {
                            values_list: vec![0],
                            value: 0,
                            special_type: *card_type,
                            board_effect: Some(|board, played_card| {
                                // Find out what the last card played was on the board
                                if let Some(last_card) = board.cards.last() {
                                    // If the last card played was a double, play the card again
                                    played_card.value = last_card.value;
                                }
                            }),
                        });
                    }
                    SpecialType::TieBreaker => {
                        let captures = regex_string.captures(card_string).unwrap();
                        let values: Vec<i8> = captures
                            .iter()
                            .skip(1)
                            .map(|x| x.unwrap().as_str().parse::<i8>().unwrap())
                            .collect();

                        return Some(Card {
                            values_list: values,
                            value: 0,
                            special_type: *card_type,
                            board_effect: None,
                        });
                    }
                }
            }
        }

        None
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut card_string = String::new();

        match self.special_type {
            SpecialType::None => {
                card_string.push_str(&format!("{:}", self.value));

                match self.value.cmp(&0) {
                    std::cmp::Ordering::Less => card_string = card_string.red().to_string(),
                    std::cmp::Ordering::Greater => card_string = card_string.green().to_string(),
                    _ => {}
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
                        card_string.push('/');
                    }
                }

                card_string = card_string.blue().to_string();
            }
            SpecialType::Invert => {
                card_string.push_str(&format!("{}&{}", self.values_list[0], self.values_list[1]));

                card_string = card_string.yellow().to_string();
            }
            SpecialType::Double => {
                if self.value != 0 {
                    card_string.push_str(&format!("{}[D]", self.value));
                } else {
                    card_string.push('D');
                }

                card_string = card_string.yellow().to_string();
            }
            SpecialType::TieBreaker => {
                // put parentheses around the list item that matches the value
                let formatted_values = self
                    .values_list
                    .iter()
                    .map(|v| {
                        if *v == self.value {
                            format!("[{:+}]", v)
                        } else {
                            format!("{:+}", v)
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("/");

                card_string = format!("{}T", formatted_values).blue().to_string();
            }
        }

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
        let new_cards = (0..4)
            .flat_map(|_| {
                (1..=10).map(|i| Card {
                    values_list: vec![i],
                    value: i,
                    special_type: SpecialType::None,
                    board_effect: None,
                })
            })
            .collect::<Vec<_>>();

        self.cards.extend(new_cards);
    }
}

impl fmt::Display for Deck {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.cards.is_empty() {
            return write!(f, "{}", "<Empty Deck>".to_string().yellow().italic());
        }

        let deck_string = self
            .cards
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ");

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
        if self.cards.is_empty() {
            return "<Empty Hand>".yellow().italic().to_string();
        }

        self.cards
            .iter()
            .map(|_| "?".to_string())
            .collect::<Vec<_>>()
            .join(", ")
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.cards.is_empty() {
            return write!(f, "{}", "<Empty Hand>".to_string().yellow().italic());
        }

        let hand_string = self
            .cards
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "{}", hand_string)
    }
}

#[derive(PartialEq, Clone)]
pub enum Status {
    Playing,
    Standing,
    Busted,
}

// player takes a mutable deck
#[derive(Clone)]
pub struct Player {
    pub hand: Hand,
    pub deck: Deck,
    pub status: Status,
}
#[derive(Clone)]
pub struct Board {
    pub cards: Vec<Card>,
}

impl Board {
    pub fn total(&self) -> i8 {
        self.cards.iter().map(|c| c.value).sum()
    }

    pub fn has_tiebreaker(&self) -> bool {
        self.cards
            .iter()
            .any(|c| c.special_type == SpecialType::TieBreaker)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.cards.is_empty() {
            return write!(f, "{}", "<Empty Board>".to_string().yellow().italic());
        }

        let board_string = self
            .cards
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "{} ({})", board_string, self.total())
    }
}
// A Game is a collection of players, boards, and a deck
#[derive(Clone)]
pub struct Game {
    pub board: [Board; 2],
    pub deck: Deck,
    pub turn: u8,
    pub winner: u8,
}

impl Game {
    pub fn new() -> Game {
        let board1 = Board { cards: vec![] };
        let board2 = Board { cards: vec![] };

        // Generate Game Deck
        let mut board_deck = Deck::new();
        board_deck.default_fill();
        board_deck.shuffle();

        Game {
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

        match (player1_distance < 0, player2_distance < 0) {
            (true, true) => None, // Both players busted
            (true, _) => Some(1), // Only Player 1 busted
            (_, true) => Some(0), // Only Player 2 busted
            _ => match player1_distance.cmp(&player2_distance) {
                std::cmp::Ordering::Less => Some(0),    // Player 1 is closer
                std::cmp::Ordering::Greater => Some(1), // Player 2 is closer
                std::cmp::Ordering::Equal => {
                    // Tiebreaker logic
                    if self.board[0].has_tiebreaker() {
                        Some(0)
                    } else if self.board[1].has_tiebreaker() {
                        Some(1)
                    } else {
                        None
                    }
                }
            },
        }
    }
}

#[derive(Clone)]
pub struct Match {
    pub games: Vec<Game>,
    pub players: [Player; 2],
    pub match_detail: MatchDetails,
}

impl Match {
    pub fn new(mut deck1: Deck, mut deck2: Deck) -> Match {
        let mut player_hand = Hand::new();
        let mut opponent_hand = Hand::new();

        for _ in 0..4 {
            player_hand.cards.push(deck1.draw().unwrap());
            opponent_hand.cards.push(deck2.draw().unwrap());
        }

        Match {
            games: vec![],
            players: [
                Player {
                    hand: player_hand,
                    deck: deck1,
                    status: Status::Playing,
                },
                Player {
                    hand: opponent_hand,
                    deck: deck2,
                    status: Status::Playing,
                },
            ],
            match_detail: MatchDetails::new(),
        }
    }

    pub fn new_game(&mut self) {
        let new_game = Game::new();

        // Reset the players' statuses
        self.players[0].status = Status::Playing;
        self.players[1].status = Status::Playing;

        // add the game to the match
        self.games.push(new_game);

        // Increment the round
        self.match_detail.round += 1;
    }

    pub fn current_game(&mut self) -> &mut Game {
        &mut self.games[self.match_detail.round - 1]
    }

    // Check which player won the match by reaching 3 points
    pub fn check_win(&mut self) -> Option<usize> {
        match self.match_detail.score {
            [3, _] => Some(0),
            [_, 3] => Some(1),
            _ => None,
        }
    }
}

impl fmt::Display for Match {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let current_game = &self.games[self.match_detail.round - 1];

        // Start with the opponent's information
        writeln!(f, "{}", "---------------------------".blue().bold())?;
        writeln!(f, "Opponent Board: {}", current_game.board[1])?;
        writeln!(
            f,
            "Opponent Hand: {}",
            self.players[1].hand.get_anonymous_hand_string()
        )?;

        // Divider
        writeln!(f, "{}", "~~~~~~~~~~~~~~~~~~~~~~~~~~~".blue().bold())?;

        // Then, your information
        writeln!(f, "Your Board: {}", current_game.board[0])?;
        writeln!(f, "Your Hand: {}", self.players[0].hand)?;

        // End with a closing line
        writeln!(f, "{}", "---------------------------".blue().bold())
    }
}

#[derive(Clone)]
pub struct MatchDetails {
    pub round: usize,
    pub score: [u8; 2],
}

impl MatchDetails {
    pub fn new() -> MatchDetails {
        MatchDetails {
            round: 0,
            score: [0, 0],
        }
    }
}

impl fmt::Display for MatchDetails {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Start with the header
        writeln!(f, "{}", "---------------------------".blue().bold())?;
        // Round information
        writeln!(f, "Round: {}", self.round.to_string().yellow().bold())?;
        // Score information
        writeln!(
            f,
            "You: {}  | Opponent: {}",
            self.score[0].to_string().green().bold(),
            self.score[1].to_string().red().bold()
        )?;
        // End with a footer
        writeln!(f, "{}", "---------------------------".blue().bold())
    }
}
