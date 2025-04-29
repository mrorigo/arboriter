//! Example of implementing Monte Carlo Tree Search (MCTS) using arboriter
//! 
//! This is a conceptual implementation showing how the tree traversal primitives
//! can be used to implement the MCTS algorithm in a clear and maintainable way.
//! MCTS is commonly used for decision-making in games and other domains with large
//! state spaces.

use arboriter::{for_tree, break_tree};
use std::f64;
use std::fmt;

// EXPLORATION_CONSTANT controls the balance between exploration and exploitation
// in the UCB1 formula for the selection phase
const EXPLORATION_CONSTANT: f64 = 1.414; // sqrt(2)
const NUM_SIMULATIONS: usize = 1000;

/// Represents a generic game state
/// In a real implementation, this would contain the specific game state details
trait GameState: Clone + Sized {
    /// Get all valid moves from this state
    fn get_valid_moves(&self) -> Vec<Move>;
    
    /// Apply a move to the current state and return the resulting state
    fn apply_move(&self, mov: &Move) -> Self;
    
    /// Check if the game is over
    fn is_terminal(&self) -> bool;
    
    /// Get the result from this terminal state (1 for win, 0 for draw, -1 for loss)
    /// from the perspective of the player who just moved
    fn get_result(&self) -> f64;
    
    /// Run a random simulation from this state until terminal state and return the result
    fn simulate(&self) -> f64 {
        let mut current_state = self.clone();
        let mut is_maximizing = true; // Tracks whose perspective we're evaluating
        
        while !current_state.is_terminal() {
            let valid_moves = current_state.get_valid_moves();
            if valid_moves.is_empty() {
                break;
            }
            
            // Select a random move
            let random_index = rand::random::<usize>() % valid_moves.len();
            let random_move = &valid_moves[random_index];
            
            current_state = current_state.apply_move(random_move);
            is_maximizing = !is_maximizing; // Switch player
        }
        
        // Return result from perspective of player who just moved
        let result = current_state.get_result();
        if is_maximizing {
            result
        } else {
            -result
        }
    }
}

/// Represents a move in the game
/// This is a generic representation - in a real implementation,
/// this would contain the specific move details for your game
#[derive(Clone, PartialEq, Eq, Hash)]
struct Move {
    id: usize,
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Move({})", self.id)
    }
}

/// Monte Carlo Tree Search Node
#[derive(Clone)]
struct MCTSNode<S: GameState> {
    // Game state represented by this node
    state: S,
    // Move that led to this state (None for root)
    action: Option<Move>,
    // Number of times this node has been visited
    visits: usize,
    // Total reward accumulated from simulations passing through this node
    total_reward: f64,
    // Children nodes created by applying valid moves
    children: Vec<MCTSNode<S>>,
    // Available moves that haven't been expanded yet
    unexpanded_moves: Vec<Move>,
    // Parent node (None for root)
    parent: Option<Box<MCTSNode<S>>>,
}

impl<S: GameState> MCTSNode<S> {
    /// Create a new MCTS node from a game state
    fn new(state: S, action: Option<Move>, parent: Option<Box<MCTSNode<S>>>) -> Self {
        let unexpanded_moves = state.get_valid_moves();
        
        MCTSNode {
            state,
            action,
            visits: 0,
            total_reward: 0.0,
            children: Vec::new(),
            unexpanded_moves,
            parent,
        }
    }
    
    /// Calculate UCB1 value for this node (used in selection phase)
    fn ucb1_value(&self, parent_visits: usize) -> f64 {
        if self.visits == 0 {
            return f64::INFINITY; // Unvisited nodes have infinite potential
        }
        
        let exploitation = self.total_reward / self.visits as f64;
        let exploration = EXPLORATION_CONSTANT * ((parent_visits as f64).ln() / self.visits as f64).sqrt();
        
        exploitation + exploration
    }
    
    /// Is this node fully expanded? (i.e., all possible moves have been tried)
    fn is_fully_expanded(&self) -> bool {
        self.unexpanded_moves.is_empty()
    }
    
    /// Is this node a leaf node? (terminal or not yet expanded)
    fn is_leaf(&self) -> bool {
        self.children.is_empty() || self.state.is_terminal()
    }
    
    /// Expand this node by selecting an unexpanded move and creating a child node
    fn expand(&mut self) -> Option<&mut MCTSNode<S>> {
        if self.unexpanded_moves.is_empty() {
            return None;
        }
        
        // Take the last unexpanded move
        let mov = self.unexpanded_moves.pop().unwrap();
        let new_state = self.state.apply_move(&mov);
        
        // Create a child node and add it to children
        let child = MCTSNode::new(
            new_state,
            Some(mov),
            None, // We'll handle parent relationships differently for simplicity
        );
        
        self.children.push(child);
        self.children.last_mut()
    }
    
    /// Update node statistics with a simulation result
    fn update(&mut self, result: f64) {
        self.visits += 1;
        self.total_reward += result;
    }
    
    /// Get the best child based on UCB1 formula (for selection)
    fn best_child(&self, exploration: bool) -> Option<usize> {
        if self.children.is_empty() {
            return None;
        }
        
        let mut best_value = f64::NEG_INFINITY;
        let mut best_index = 0;
        
        for (i, child) in self.children.iter().enumerate() {
            let value = if exploration {
                child.ucb1_value(self.visits)
            } else {
                // For final move selection, we typically use highest average reward
                child.total_reward / child.visits as f64
            };
            
            if value > best_value {
                best_value = value;
                best_index = i;
            }
        }
        
        Some(best_index)
    }
}

/// Implement Monte Carlo Tree Search algorithm using arboriter
struct MCTS<S: GameState> {
    root: MCTSNode<S>,
}

impl<S: GameState> MCTS<S> {
    fn new(initial_state: S) -> Self {
        MCTS {
            root: MCTSNode::new(initial_state, None, None),
        }
    }
    
    /// Run MCTS for a specified number of iterations and return the best move
    fn search(&mut self, iterations: usize) -> Option<Move> {
        for _ in 0..iterations {
            // 1. Selection phase - find a promising leaf node
            let selected_path = self.selection();
            
            // Get the last node in the path (the selected node)
            if let Some(selected_node) = selected_path.last() {
                let _node_index = selected_node.0; // Unused but kept for clarity
                let mut current_node = &mut self.root;
                
                // Navigate to the selected node
                for &(idx, _) in selected_path.iter().skip(1) {
                    current_node = &mut current_node.children[idx];
                }
                
                // 2. Expansion phase
                let expanded_node = if !current_node.is_fully_expanded() && !current_node.state.is_terminal() {
                    current_node.expand()
                } else {
                    Some(current_node)
                };
                
                if let Some(node) = expanded_node {
                    // 3. Simulation phase
                    let result = node.state.simulate();
                    
                    // 4. Backpropagation phase
                    self.backpropagation(&selected_path, result);
                }
            }
        }
        
        // Return the best move (using exploitation only)
        if let Some(best_child_index) = self.root.best_child(false) {
            self.root.children[best_child_index].action.clone()
        } else {
            None
        }
    }
    
    /// Selection phase of MCTS - use for_tree to traverse down the tree
    /// Returns a path (sequence of (child_index, visits)) from root to selected node
    fn selection(&self) -> Vec<(usize, usize)> {
        let mut path = Vec::new();
        path.push((0, self.root.visits)); // Start with root
        
        // Use arboriter to traverse down the tree
        for_tree!(node_data in (0, &self.root); 
                 |(_, node)| node.is_leaf() == false; 
                 |(_current_idx, current_node)| {
            let best_child_idx = match current_node.best_child(true) {
                Some(idx) => idx,
                None => return Vec::new(),
            };
            vec![(best_child_idx, &current_node.children[best_child_idx])]
        } => {
            let (idx, node) = *node_data;
            path.push((idx, node.visits));
            
            // Terminal state check (we can't expand terminal states)
            if node.state.is_terminal() {
                break_tree!();
            }
            
            // If the node is not fully expanded, we can stop here
            if !node.is_fully_expanded() {
                break_tree!();
            }
        });
        
        path
    }
    
    /// Backpropagation phase of MCTS - update statistics along the path
    fn backpropagation(&mut self, path: &[(usize, usize)], result: f64) {
        // Update root
        self.root.update(result);
        
        // Update nodes along the path
        let mut current_node = &mut self.root;
        for &(child_idx, _) in path.iter().skip(1) {
            current_node = &mut current_node.children[child_idx];
            current_node.update(result);
        }
    }
    
    /// Print the current tree (useful for debugging)
    fn print_tree(&self) {
        println!("MCTS Tree Statistics:");
        
        // Use for_tree to traverse and print the entire tree
        for_tree!(node_data in (0, &self.root, 0); 
                 |_| true; 
                 |(_, node, depth)| {
            node.children.iter().enumerate().map(|(i, child)| 
                (i, child, depth + 1)
            ).collect()
        } => {
            let (_i, node, depth) = *node_data;
            let indent = "  ".repeat(depth);
            let move_str = match &node.action {
                Some(action) => format!("{:?}", action),
                None => "Root".to_string(),
            };
            
            println!("{}{} - visits: {}, value: {:.3}", 
                indent, 
                move_str,
                node.visits, 
                if node.visits > 0 { node.total_reward / node.visits as f64 } else { 0.0 }
            );
        });
    }
}

/// Simple TicTacToe implementation to demonstrate MCTS
#[derive(Clone, PartialEq, Eq)]
struct TicTacToe {
    board: [Option<Player>; 9],
    current_player: Player,
    moves_played: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Player {
    X,
    O,
}

impl TicTacToe {
    fn new() -> Self {
        TicTacToe {
            board: [None; 9],
            current_player: Player::X,
            moves_played: 0,
        }
    }
    
    fn check_winner(&self) -> Option<Player> {
        // Check rows
        for i in 0..3 {
            if self.board[i*3] != None && self.board[i*3] == self.board[i*3+1] && self.board[i*3] == self.board[i*3+2] {
                return self.board[i*3];
            }
        }
        
        // Check columns
        for i in 0..3 {
            if self.board[i] != None && self.board[i] == self.board[i+3] && self.board[i] == self.board[i+6] {
                return self.board[i];
            }
        }
        
        // Check diagonals
        if self.board[0] != None && self.board[0] == self.board[4] && self.board[0] == self.board[8] {
            return self.board[0];
        }
        if self.board[2] != None && self.board[2] == self.board[4] && self.board[2] == self.board[6] {
            return self.board[2];
        }
        
        None
    }
    
    fn print_board(&self) {
        println!("Current board:");
        for i in 0..3 {
            for j in 0..3 {
                let symbol = match self.board[i*3 + j] {
                    Some(Player::X) => "X",
                    Some(Player::O) => "O",
                    None => ".",
                };
                print!("{} ", symbol);
            }
            println!();
        }
    }
}

impl GameState for TicTacToe {
    fn get_valid_moves(&self) -> Vec<Move> {
        let mut moves = Vec::new();
        for i in 0..9 {
            if self.board[i].is_none() {
                moves.push(Move { id: i });
            }
        }
        moves
    }
    
    fn apply_move(&self, mov: &Move) -> Self {
        let mut new_state = self.clone();
        new_state.board[mov.id] = Some(self.current_player);
        new_state.current_player = match self.current_player {
            Player::X => Player::O,
            Player::O => Player::X,
        };
        new_state.moves_played += 1;
        new_state
    }
    
    fn is_terminal(&self) -> bool {
        self.check_winner().is_some() || self.moves_played == 9
    }
    
    fn get_result(&self) -> f64 {
        // Get result from perspective of player who just moved (not current_player)
        if let Some(winner) = self.check_winner() {
            if winner == self.current_player {
                // If current player won, the previous player lost
                -1.0
            } else {
                // If current player lost, the previous player won
                1.0
            }
        } else {
            // Draw
            0.0
        }
    }
}

fn main() {
    println!("Monte Carlo Tree Search Example using Arboriter");
    println!("==============================================");
    
    // Create initial game state
    let initial_state = TicTacToe::new();
    
    // Play a simple game against MCTS AI
    let mut current_state = initial_state;
    
    while !current_state.is_terminal() {
        current_state.print_board();
        
        // MCTS makes a move
        let mut mcts = MCTS::new(current_state.clone());
        let best_move = mcts.search(NUM_SIMULATIONS);
        
        println!("MCTS statistics after {} simulations:", NUM_SIMULATIONS);
        mcts.print_tree();
        
        if let Some(mov) = best_move {
            println!("MCTS chooses move: {:?}", mov);
            current_state = current_state.apply_move(&mov);
        } else {
            println!("No valid moves available");
            break;
        }
        
        // Check if game is over
        if current_state.is_terminal() {
            current_state.print_board();
            match current_state.check_winner() {
                Some(Player::X) => println!("Player X wins!"),
                Some(Player::O) => println!("Player O wins!"),
                None => println!("It's a draw!"),
            }
            break;
        }
        
        // Human player's turn (simplified as random move for this example)
        let valid_moves = current_state.get_valid_moves();
        if !valid_moves.is_empty() {
            // In a real implementation, you would prompt for user input here
            let random_index = rand::random::<usize>() % valid_moves.len();
            let human_move = &valid_moves[random_index];
            println!("Human (random) chooses move: {:?}", human_move);
            current_state = current_state.apply_move(human_move);
        }
        
        // Check if game is over after human move
        if current_state.is_terminal() {
            current_state.print_board();
            match current_state.check_winner() {
                Some(Player::X) => println!("Player X wins!"),
                Some(Player::O) => println!("Player O wins!"),
                None => println!("It's a draw!"),
            }
            break;
        }
    }
}