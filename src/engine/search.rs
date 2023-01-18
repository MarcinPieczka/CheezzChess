
use chess::{Board, ChessMove, Color, MoveGen, Game};
use log::info;
use super::tree::Tree;
use std::rc::{Weak, Rc};


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

        let mut moves = vec![];
        loop {
            if self.tree.current.borrow().data.depth < max_depth {
                if self.tree.current.borrow().data.potential_next_moves.is_none() {
                    let board = board_from_moves(self.board.clone(), &moves);
                    let legal_moves = MoveGen::new_legal(&board).collect();
                    self.tree.current.borrow_mut().data.potential_next_moves = Some(legal_moves);
                }
                let next_move = self.tree.current.borrow_mut().data.potential_next_moves.as_mut().unwrap().pop();
                // match &mut Rc::clone(&self.tree.current).borrow_mut().data.potential_next_moves {
                //     Some(potential_moves) if potential_moves == &[] => {
                //         if self.tree.has_no_child() {}
                //         // There are no more moves either because we used them all
                //         // or there vere none to begin with.
                //         //
                //         // If there are children, then the a/b are set correctly
                //         // and we can move up with the a/b
                //         //
                //         // If there were no potential moves, then we have to evaluate this position
                //     }
                //     Some(potential_moves) => {
                //         let next_move = potential_moves.pop().unwrap();
                //         let alpha = self.tree.current.borrow().data.alpha;
                //         let beta = self.tree.current.borrow().data.beta;
                //         let depth = self.tree.current.borrow().data.depth + 1;

                //         self.tree.add_child(Position::new(Some(next_move), alpha, beta, depth));
                //         self.tree.goto_last_child();

                //         moves.push(next_move);
                //     }
                //     None => {
                //         panic!("this field should have been initialized already")
                //     }
                                //         let next_move = potential_moves.pop().unwrap();
                match next_move {
                    Some(mv) => {
                        let alpha = self.tree.current.borrow().data.alpha;
                        let beta = self.tree.current.borrow().data.beta;
                        let depth = self.tree.current.borrow().data.depth + 1;
        
                        self.tree.add_child(Position::new(next_move, alpha, beta, depth));
                        self.tree.goto_last_child();
        
                        moves.push(mv); 
                    },
                    None => {
                        if self.tree.has_no_child() {}
                        // There are no more moves either because we used them all
                        // or there vere none to begin with.
                        //
                        // If there are children, then the a/b are set correctly
                        // and we can move up with the a/b
                        //
                        // If there were no potential moves, then we have to evaluate this position
                   
                    }
                }
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_running_search() {
        let board = Game::new().current_position();
        let mut search = Search::new(&board, Color::White);
        println!("{:?}", board.side_to_move());
        search.run(4, None, None);
    }
}