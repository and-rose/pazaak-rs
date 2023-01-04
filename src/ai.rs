use std::{collections::HashMap, rc::Weak};

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

fn select() {
    
}

fn expand() {

}

fn simulate() {

}

fn back_propagate() {

}

fn best_ucb_score(node: Node) {
    let mut best_node = node;
    let mut max_ucb = 0.0;

    for child in node.children {
        let child_ucb = calculate_ucb_score(child.score, child.visit, child.parent.upgrade().unwrap().visit);

        if child_ucb >= max_ucb {
            max_ucb = child_ucb;
            best_node = child;
        }        
    }
    return best_node;
}

fn calculate_ucb_score(win_score: f32, visit_score:f32, parent_visit_score: f32) -> f32 {
    if visit_score < 1.0 {
        return f32::INFINITY;
    }

    let constant = 1.4;
    let win_ratio = win_score/ visit_score;
    let exploration = (parent_visit_score.log10() / visit_score).sqrt();
    return win_ratio + constant * exploration;
}

struct Node {
    pub game: Game,
    pub hand: Hand,
    pub action: Action,
    pub parent: Weak<Node>, //TODO: Weak reference to parent
    pub children: Vec<Node>,
    pub visit: f32,
    pub score: f32,
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
