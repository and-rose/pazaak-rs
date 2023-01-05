use std::{borrow::BorrowMut, collections::HashMap, ops::Deref, rc::Weak};

use rand::seq::{IteratorRandom, SliceRandom};

use crate::{
    cards::{Card, Game, Hand, Match},
    util::Action,
};

/// Simulations is repeated as long as the time to make a move permits.
/// The implementation here can be changed to be time based instead of a fixed number of simulations.
pub fn get_best_move(pazaak_match: &Match, simulations: i8) -> Option<Vec<NodeAction>> {
    let match_number = pazaak_match.match_detail.round - 1;
    let current_game = &pazaak_match.games[match_number];
    let ai_board = &current_game.board[1];
    let ai_hand = &pazaak_match.players[1].hand;

    let root_node = Node {
        game: current_game.clone(),
        hand: ai_hand.clone(),
        parent: None,
        children: HashMap::new(),
        visit: 0.0,
        score: 0.0,
    };

    for _ in 0..simulations {
        let mut node = &root_node.clone();
        node = select(node);
        expand(&node);
        let score = simulate(&node);
        backpropagate(node.clone(), score);
    }

    let best_node = best_ucb_score(&root_node);

    let mut actions = vec![];

    let mut current_node = best_node;

    // Get the next action that leads to the best node
    while current_node.parent.is_some() {
        match &current_node.parent {
            Some(parent) => {
                let parent_node = &*parent;
                for (action, child) in parent_node.children.iter() {
                    if child == current_node {
                        actions.push(action.clone());
                    }
                }

                current_node = &parent_node;
            }
            None => {
                break;
            }
        }
    }

    return None;
}

fn select(node: &Node) -> &Node {
    let mut current_node = node;

    while current_node.children.len() > 0 {
        current_node = best_ucb_score(&current_node);
    }

    return current_node;
}

fn expand(node: &Node) {
    let mut current_node = node;

    // Add all available actions to a vector
    let mut actions = vec![];
    for i in 1..=10 {
        actions.push(NodeAction::EndTurn(i));
    }

    actions.push(NodeAction::Stand);

    for card in current_node.hand.cards.iter() {
        actions.push(NodeAction::Play(card.clone()));
    }

    let mut children = vec![];

    for action in actions {
        let mut new_game = current_node.game.clone();
        let mut new_hand = current_node.hand.clone();

        match action {
            NodeAction::EndTurn(drawn_value) => {
                new_game.board[1].cards.push(Card::new(drawn_value));
            }
            NodeAction::Play(card) => {
                new_game.board[1].cards.push(card);
                // Remove card from hand - this is a bit of a hack - finding the index
                let mut index = 0;
                for i in 0..new_hand.cards.len() {
                    let card = new_hand.cards[i].clone();
                    if new_hand.cards[i].value == card.value {
                        index = i;
                        break;
                    }
                }

                new_hand.cards.remove(index);
            }
            NodeAction::Stand => {
                // Do nothing
            }
        }

        let child = Node {
            game: new_game,
            hand: new_hand,
            parent: Some(Box::new(current_node.clone())),
            children: HashMap::new(),
            visit: 0.0,
            score: 0.0,
        };

        children.push(child);
    }
}

fn simulate(node: &Node) -> f32 {
    let mut current_node = node;

    let mut rng = rand::thread_rng();
    let mut score = 0.0;

    while current_node.children.len() > 0 {
        let child = current_node.children.values().choose(&mut rng).unwrap();
        score += child.score;
        current_node = child;
    }

    // Check if the game was won
    let winner = node.game.check_win();
    if winner.is_some() {
        if winner.unwrap() == 1 {
            score += 5.0;
        } else {
            score -= 5.0;
        }
    }

    return score;
}

fn backpropagate(node: Node, score: f32) {
    let mut current_node = node;

    while current_node.parent.is_some() {
        current_node.visit += 1.0;
        current_node.score += score;
        match current_node.parent {
            Some(parent) => {
                current_node = *parent;
            }
            None => {
                break;
            }
        }
    }
}

fn best_ucb_score(node: &Node) -> &Node {
    let parent_node = node;
    let mut best_node = node;
    let mut max_ucb = 0.0;

    for child in parent_node.children.values() {
        let ucb = calculate_ucb_score(child.score, child.visit, parent_node.visit);
        if ucb > max_ucb {
            max_ucb = ucb;
            best_node = child;
        }
    }

    return best_node;
}

fn calculate_ucb_score(win_score: f32, visit_score: f32, parent_visit_score: f32) -> f32 {
    if visit_score < 1.0 {
        return f32::INFINITY;
    }

    let constant = 1.4;
    let win_ratio = win_score / visit_score;
    let exploration = (parent_visit_score.log10() / visit_score).sqrt();
    return win_ratio + constant * exploration;
}

#[derive(Clone, Debug)]
pub enum NodeAction {
    EndTurn(i8),
    Play(Card),
    Stand,
}

#[derive(Clone)]
struct Node {
    pub game: Game,
    pub hand: Hand,
    pub parent: Option<Box<Node>>,
    pub children: HashMap<Vec<NodeAction>, Node>,
    pub visit: f32,
    pub score: f32,
    // In games where draws are possible,
    // a draw causes the numerator for both
    // black and white to be incremented by 0.5 and the denominator by 1
}

impl Node {
    pub fn select_random_child(&self) -> Option<&Node> {
        let mut rng = rand::thread_rng();
        return self.children.values().choose(&mut rng);
    }

    pub fn most_visited_child(&self) -> Option<&Node> {
        return self
            .children
            .values()
            .max_by(|a, b| a.visit.partial_cmp(&b.visit).unwrap());
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        return self.game == other.game && self.hand == other.hand;
    }
}

// End turn has 10 outcomes 1-10
// Play has x outcomes based on cards in hand
// Stand has 1 outcome with no children
