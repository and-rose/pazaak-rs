use std::collections::HashMap;

use rand::seq::SliceRandom;

use crate::{cards::{Match, Game, Hand}, util::Action};

/// Simulations is repeated as long as the time to make a move permits.
/// The implementation here can be changed to be time based instead of a fixed number of simulations.
pub fn get_best_move(pazaak_match: &Match, simulations: i8) {
    let match_number = pazaak_match.match_detail.round - 1;
    let current_game = &pazaak_match.games[match_number];
    let ai_board = &current_game.board[1];
    let ai_hand = &pazaak_match.players[1].hand;
}

fn expand() {
    
}

fn simulate() {

}

fn back_propagate() {

}

struct Node {
    pub game: Game,
    pub hand: Hand,
    pub parent: HashMap<Action, Node>, // Action also needs to track which card if a card is played
    pub children: Vec<Node>,
    pub visit: i8, // denominator
    pub score: i8, // numerator
    // In games where draws are possible,
    // a draw causes the numerator for both
    // black and white to be incremented by 0.5 and the denominator by 1
}

impl Node {
    pub fn select_random_child(&self) -> Option<&Node> {
        let mut rng = rand::thread_rng();
        return self.children.choose(&mut rng);
    }

    pub fn most_visited_child(&self) -> Option<&Node> {
        return self.children.iter().max_by(|a, b| a.visit.cmp(&b.visit));
    }
}


// End turn has 10 outcomes 1-10
// Play has x outcomes based on cards in hand
// Stand has 1 outcome with no children
