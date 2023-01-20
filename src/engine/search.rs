use chess::{Board, ChessMove, Color, Game, MoveGen, Piece, Square};

use super::tree::Tree;
use crate::engine::eval::{eval, eval_with_children};
use log::info;
use std::cmp::{max, min};
use std::rc::{Rc, Weak};

pub struct Position {
    chess_move: Option<ChessMove>,
    potential_next_moves: Option<Vec<ChessMove>>,
    next_best: Option<usize>,
    alpha: i16,
    beta: i16,
    depth: u8,
}

impl Position {
    pub fn new(chess_move: Option<ChessMove>, alpha: i16, beta: i16, depth: u8) -> Position {
        Position {
            chess_move: chess_move,
            potential_next_moves: None,
            next_best: None,
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

    pub fn run(&mut self, max_depth: u8, alpha: Option<i16>, beta: Option<i16>) -> ChessMove{
        let mut number_of_pruned = 0;
        let mut number_of_evaluated = 0;
        match alpha {
            Some(val) => {
                self.tree.root.borrow_mut().data.alpha = val;
            }
            None => {}
        }
        match beta {
            Some(val) => {
                self.tree.root.borrow_mut().data.alpha = val;
            }
            None => {}
        }
        let mut color_to_move_correction = 0;
        if self.board.side_to_move() != self.color {
            color_to_move_correction += 1;
        }
        if self.board.side_to_move() == Color::Black {
            color_to_move_correction += 1;
        }
        let mut i = 0;
        let mut moves = vec![];
        loop {
            i += 1;
            if i > 2i32.pow(20) {
                //println!("reached limit");
                break;
            }
            //println!("{:?}", &moves);
            if self.tree.current.borrow().data.depth < max_depth {
                //println!("depth is not max");
                if self
                    .tree
                    .current
                    .borrow()
                    .data
                    .potential_next_moves
                    .is_none()
                {
                    //println!("potential moves not initialized");
                    let board = board_from_moves(self.board.clone(), &moves);
                    let legal_moves = MoveGen::new_legal(&board).collect();
                    self.tree.current.borrow_mut().data.potential_next_moves = Some(legal_moves);
                }
                let alpha = self.tree.current.borrow().data.alpha;
                let beta = self.tree.current.borrow().data.beta;
                if alpha > beta{
                    number_of_pruned += 1;
                    self.move_up(&mut moves);
                }

                let next_move = self
                    .tree
                    .current
                    .borrow_mut()
                    .data
                    .potential_next_moves
                    .as_mut()
                    .unwrap()
                    .pop();

                match next_move {
                    Some(mv) => {
                        //println!("there is next move");
                        let alpha = self.tree.current.borrow().data.alpha;
                        let beta = self.tree.current.borrow().data.beta;
                        let depth = self.tree.current.borrow().data.depth + 1;

                        self.tree
                            .add_child(Position::new(next_move, alpha, beta, depth));
                        self.tree.goto_last_child();

                        moves.push(mv);
                    }
                    None => {
                        //println!("there is no next move");
                        number_of_evaluated += 1;

                        let eval = eval(&self.board, &moves);
                        let child_idx = self.tree.current.borrow().index;

                        if self.corrected_depth(color_to_move_correction) % 2 == 0 {
                            if self.tree.has_no_child() {
                                let alpha = max(eval, self.tree.current.borrow().data.alpha);
                                self.tree.current.borrow_mut().data.alpha = alpha;
                                    
                            }
                            let alpha = self.tree.current.borrow().data.alpha;
                            if self.move_up(&mut moves) {
                                if alpha < self.tree.current.borrow().data.beta {
                                    self.show_board_from_moves(&moves);
                                    self.tree.current.borrow_mut().data.beta = alpha;
                                    self.tree.current.borrow_mut().data.next_best = child_idx;
                                }
                            } else {
                                break;
                            }
                        } else {
                            if self.tree.has_no_child() {
                                let beta = min(eval, self.tree.current.borrow().data.beta);
                                self.tree.current.borrow_mut().data.beta = beta;
                            }
                            let beta = self.tree.current.borrow().data.beta;
                            if self.move_up(&mut moves) {
                                if beta > self.tree.current.borrow().data.alpha {
                                    self.show_board_from_moves(&moves);
                                    self.tree.current.borrow_mut().data.alpha = beta;
                                    self.tree.current.borrow_mut().data.next_best = child_idx;
                                }
                            } else {
                                break;
                            }
                        }
                    }
                }
            } else {
                number_of_evaluated += 1;
                let (min_eval, max_eval) = eval_with_children(&self.board, &moves);
                let corrected_depth = self.corrected_depth(color_to_move_correction);
                let child_idx = self.tree.current.borrow().index;
                if corrected_depth % 2 == 0 {
                    let alpha = max(self.tree.current.borrow().data.alpha, max_eval);
                    self.tree.current.borrow_mut().data.alpha = alpha;
                    if self.move_up(&mut moves) {
                        if alpha < self.tree.current.borrow().data.beta {
                            self.show_board_from_moves(&moves);
                            self.tree.current.borrow_mut().data.beta = alpha;
                            self.tree.current.borrow_mut().data.next_best = child_idx;
                        }
                    } else {
                        break;
                    }
                } else {
                    let beta = max(self.tree.current.borrow().data.beta, min_eval);
                    self.tree.current.borrow_mut().data.beta = beta;
                    if self.move_up(&mut moves) {
                        if beta > self.tree.current.borrow().data.alpha {
                            self.show_board_from_moves(&moves);
                            self.tree.current.borrow_mut().data.alpha = beta;
                            self.tree.current.borrow_mut().data.next_best = child_idx;
                        }
                    } else {
                        break;
                    }
                }
            }
        }
        //println!(
        //     "{:?}",
        //     (
        //         self.tree.root.borrow().data.alpha,
        //         self.tree.root.borrow().data.beta,
        //     )
        // );
        loop {
            info!("Checking if has parent");
            if self.tree.has_parent(){
                info!("Going to parent");
                self.tree.goto_parent();
            } else {
                break;
            } 
        }

        let next_move_idx = self.tree.current.borrow().data.next_best;
        self.show_board_from_moves(&moves);
        info!("number of pruned: {}", number_of_pruned);
        info!("number of evaluated: {}", number_of_evaluated);


        self.tree.goto_child(next_move_idx.unwrap());
        let next_move = self.tree.current.borrow().data.chess_move.unwrap();
        next_move
    }

    fn corrected_depth(&self, color_to_move_correction: u8) -> u8 {
        self.tree.current.borrow().data.depth + color_to_move_correction
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

    fn show_board_from_moves(&self, moves: &Vec<ChessMove>) {
        if moves.len() < 2{
            info!("depth: {}", moves.len());
            info!("index: {:?}", self.tree.root.borrow().data.next_best);
            show_board(board_from_moves(self.board, moves));
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
    use chess::CastleRights;

    use crate::engine::utils::board_from_textboard;

    use super::*;

    #[test]
    fn test_running_search() {
        let board = Game::new().current_position();
        let mut search = Search::new(&board, Color::White);
        //println!("{:?}", board.side_to_move());
        search.run(2, None, None);
    }

    #[test]
    fn test_performing_checkmate_in_one() {
        let textboard = r#"
        8|   |   |   |   |   |   |   | ♔ |
        7|   |   |   |   |   |   |   | ♟︎ |
        6|   |   |   |   |   |   | ♟︎ | ♚ |
        5|   |   |   |   |   |   |   |   |
        4|   |   |   |   |   |   |   |   |
        3|   |   |   |   |   |   |   |   |
        2|   |   |   |   |   |   |   |   |
        1|   |   |   |   |   |   |   |   |
        a   b   c   d   e   f   g   h 
        "#;
        let board = board_from_textboard(
            textboard,
            CastleRights::NoRights,
            CastleRights::NoRights,
            Color::White,
        );
        let mut search = Search::new(&board, Color::White);
        //println!("{:?}", board.side_to_move());
        search.run(2, None, None);
    }

    #[test]
    fn test_avoiding_checkmate_in_one() {
        let textboard = r#"
        8|   |   |   |   |   |   |   | ♔ |
        7|   |   |   |   |   | ♙ | ♙ | ♙ |
        6|   |   |   |   |   |   |   |   |
        5|   |   |   |   |   |   |   |   |
        4|   |   |   |   |   |   |   |   |
        3|   |   |   |   |   |   |   |   |
        2|   |   |   |   |   |   |   |   |
        1|   |   |   |   | ♜ | ♚ |   |   |
        a   b   c   d   e   f   g   h 
        "#;
        let board = board_from_textboard(
            textboard,
            CastleRights::NoRights,
            CastleRights::NoRights,
            Color::Black,
        );
        let mut search = Search::new(&board, Color::Black);
        println!("{:?}", search.run(2, None, None));
        search.run(2, None, None);
    }
}
