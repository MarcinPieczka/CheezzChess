
use chess::{Board, ChessMove, Color, MoveGen, Piece, Square, Game};

use log::info;
use super::tree::Tree;
use std::rc::{Weak, Rc};
use crate::engine::eval::eval_with_children;

pub struct Position {
    chess_move: Option<ChessMove>,
    potential_next_moves: Option<Vec<ChessMove>>,
    alpha: i16,
    beta: i16,
    depth: u8,
}

impl Position {
    pub fn new(chess_move: Option<ChessMove>, alpha: i16, beta: i16, depth: u8) -> Position {
        Position {
            chess_move: chess_move,
            potential_next_moves: None,
            alpha: alpha,
            beta: beta,
            depth: depth,
        }
    }
}

pub struct Search {
    tree: Tree<Position>,
    color: Color,
    board: Board,
}

impl Search {
    pub fn new(board: &Board, color: Color) -> Search {
        info!("Creating Search with color {:?}", board.side_to_move());
        Search {
            tree: Tree::new(Position::new(None, i16::MIN, i16::MAX, 0)),
            color: color,
            board: board.clone(),
        }
    }

    pub fn run(&mut self, max_depth: u8, alpha: Option<i16>, beta: Option<i16>) {
        match alpha {
            Some(val) => {self.tree.root.borrow_mut().data.alpha = val;},
            None => {}
        }
        match beta {
            Some(val) => {self.tree.root.borrow_mut().data.alpha = val;},
            None => {}
        }
        let mut color_to_move_correction = 0;
        if self.board.side_to_move() != self.color {
            color_to_move_correction = 1;
        }
        let mut i = 0;
        let mut moves = vec![];
        loop {
            i += 1;
            if i > 1000000000 {
                println!("reached limit");
                break;
            }
            println!("{:?}", &moves);
            if self.tree.current.borrow().data.depth < max_depth {
                println!("depth is not max");
                if self.tree.current.borrow().data.potential_next_moves.is_none() {
                    println!("potential moves not initialized");
                    let board = board_from_moves(self.board.clone(), &moves);
                    let legal_moves = MoveGen::new_legal(&board).collect();
                    self.tree.current.borrow_mut().data.potential_next_moves = Some(legal_moves);
                }
                let next_move = self.tree.current.borrow_mut().data.potential_next_moves.as_mut().unwrap().pop();

                match next_move {
                    Some(mv) => {
                        println!("there is next move");
                        let alpha = self.tree.current.borrow().data.alpha;
                        let beta = self.tree.current.borrow().data.beta;
                        let depth = self.tree.current.borrow().data.depth + 1;
        
                        self.tree.add_child(Position::new(next_move, alpha, beta, depth));
                        self.tree.goto_last_child();
        
                        moves.push(mv); 
                    },
                    None => {
                        println!("there is no next move");
                        if self.tree.has_no_child() {
                            println!("we are in the leaf, but not at max depth");
                            // here we evaluate the current chess move only
                            // as it is either checkmate or stalemate
                        }
                        // here we should set the alpha or beta
                        println!("No possible move");
                        if !self.move_up(&mut moves) {
                            break;
                        }
                        // There are no more moves either because we used them all
                        // or there vere none to begin with.
                        //
                        // If there are children, then the a/b are set correctly
                        // and we can move up with the a/b
                        //
                        // If there were no potential moves, then we have to evaluate this position
                    }
                }
            } else {
                let (min, max) = eval_with_children(&self.board, &moves);
                if (self.tree.current.borrow().data.depth + color_to_move_correction) % 2 == 0 {
                    if !self.move_up(&mut moves) {
                        // here save alpha to current node (the root node)
                        break;
                    } else {
                        // here save alpha to current node (the root node)
                    }
                } else {
                    if !self.move_up(&mut moves) {
                        // here save beta to current node (the root node)
                        break;
                    } else {
                        // here save beta to current node (the root node)
                    }
                }
                // Here is main evaluation place
                // We evaluate all possible moves and get best/worst from there
                // and save alpha/beta to parent
                if !self.move_up(&mut moves) {
                    break;
                }
            }
        }
    }

    fn move_up(&mut self, moves: &mut Vec<ChessMove>) -> bool {
        if self.tree.has_parent() {
            self.tree.goto_parent();
            moves.pop();
            true
        } else {
            false
        }
    }
}

fn board_from_moves(initial_board: Board, moves: &Vec<ChessMove>) -> Board {
    let mut board = initial_board.clone();
    for mv in moves.iter() {
        board = board.make_move_new(*mv);
    }
    board
}

pub fn show_board(board: Board) {
    for l in (0..8).rev() {
        let mut line = "".to_string();
        for f in 0..8 {
            match board.piece_on(unsafe { Square::new(f + l * 8) }) {
                Some(Piece::King) => line += "|K ",
                Some(Piece::Queen) => line += "|Q ",
                Some(Piece::Rook) => line += "|R ",
                Some(Piece::Bishop) => line += "|B ",
                Some(Piece::Knight) => line += "|Kn",
                Some(Piece::Pawn) => line += "|p ",
                None => line += "|  ",
            }
        }
        info!("{}", line);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_running_search() {
        let board = Game::new().current_position();
        let mut search = Search::new(&board, Color::White);
        println!("{:?}", board.side_to_move());
        search.run(2, None, None);
    }
}
