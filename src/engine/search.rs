use chess::{Board, ChessMove, Color, Game, MoveGen, Piece, Square};

use super::tree::Tree;
use crate::engine::eval::{eval, eval_with_children};
use crate::engine::utils::show_board;
use std::cmp::{max, min};
use std::rc::{Rc, Weak};
use std::str::FromStr;

#[cfg(not(test))] 
use log::{info, warn};
 
#[cfg(test)]
use std::{println as info, println as warn};

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
        info!("Creating Search with color {:?}", color);
        show_board(*board);
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
        let mut depth_correction = 0;
        if self.board.side_to_move() != self.color {
            info!("using depth correction");
            depth_correction = 1;
        } else {
            info!("not using depth correction");
        }
        let mut i = 0;
        let mut moves = vec![];
        loop {
            i += 1;
            if i > 2i32.pow(20) {
                //println!("reached limit");
                break;
            }
            if self.tree.current.borrow().data.depth < max_depth {
                if self
                    .tree
                    .current
                    .borrow()
                    .data
                    .potential_next_moves
                    .is_none()
                {
                    let board = board_from_moves(self.board.clone(), &moves);
                    let legal_moves = get_possible_moves(&board);
                    self.tree.current.borrow_mut().data.potential_next_moves = Some(legal_moves);
                }
                let alpha = self.tree.current.borrow().data.alpha;
                let beta = self.tree.current.borrow().data.beta;
                if alpha >= beta{
                    number_of_pruned += 1;
                    let child_idx = self.tree.current.borrow().index;
                    if self.move_up(&mut moves) {
                        if self.corrected_depth(depth_correction) % 2 == 0 {
                            if alpha > self.tree.current.borrow().data.alpha {
                                self.tree.current.borrow_mut().data.alpha = alpha;
                                self.tree.current.borrow_mut().data.next_best = child_idx;
                                self.show_board_from_moves(&moves);
                            }
                        } else {
                            if beta > self.tree.current.borrow().data.beta {
                                self.tree.current.borrow_mut().data.beta = beta;
                                self.tree.current.borrow_mut().data.next_best = child_idx;
                                self.show_board_from_moves(&moves);
                            }
                        }
                    }
                    // here I think the parent should be updated with beta
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
                        let alpha = self.tree.current.borrow().data.alpha;
                        let beta = self.tree.current.borrow().data.beta;
                        let depth = self.tree.current.borrow().data.depth + 1;

                        self.tree
                            .add_child(Position::new(next_move, alpha, beta, depth));
                        self.tree.goto_last_child();

                        moves.push(mv);
                    }
                    None => {
                        number_of_evaluated += 1;

                        let eval = eval(&self.board, &moves, self.color);
                        let child_idx = self.tree.current.borrow().index;

                        if self.corrected_depth(depth_correction) % 2 == 0 {
                            if self.tree.has_no_child() {
                                // here should only be checkmate or stalemate
                                let alpha = max(eval, self.tree.current.borrow().data.alpha);
                                self.tree.current.borrow_mut().data.alpha = alpha;
                                    
                            }
                            let alpha = self.tree.current.borrow().data.alpha;
                            if self.move_up(&mut moves) {
                                if alpha < self.tree.current.borrow().data.beta {
                                    self.tree.current.borrow_mut().data.beta = alpha;
                                    self.tree.current.borrow_mut().data.next_best = child_idx;
                                    self.show_board_from_moves(&moves);
                                }
                            } else {
                                break;
                            }
                        } else {
                            if self.tree.has_no_child() {
                                // here should only be checkmate or stalemate
                                let beta = min(eval, self.tree.current.borrow().data.beta);
                                self.tree.current.borrow_mut().data.beta = beta;
                            }
                            let beta = self.tree.current.borrow().data.beta;
                            if self.move_up(&mut moves) {
                                if beta > self.tree.current.borrow().data.alpha {
                                    self.tree.current.borrow_mut().data.alpha = beta;
                                    self.tree.current.borrow_mut().data.next_best = child_idx;
                                    self.show_board_from_moves(&moves);
                                }
                            } else {
                                break;
                            }
                        }
                    }
                }
            } else {
                number_of_evaluated += 1;
                let (min_eval, max_eval) = eval_with_children(&self.board, &moves, self.color);
                let child_idx = self.tree.current.borrow().index;

                if self.corrected_depth(depth_correction) % 2 == 0 {
                    let alpha = max(self.tree.current.borrow().data.alpha, max_eval);
                    self.tree.current.borrow_mut().data.alpha = alpha;
                    if self.move_up(&mut moves) {
                        if alpha < self.tree.current.borrow().data.beta {
                            self.tree.current.borrow_mut().data.beta = alpha;
                            self.tree.current.borrow_mut().data.next_best = child_idx;
                            self.show_board_from_moves(&moves);
                        }
                    } else {
                        break;
                    }
                } else {
                    let beta = min(self.tree.current.borrow().data.beta, min_eval);
                    self.tree.current.borrow_mut().data.beta = beta;
                    if self.move_up(&mut moves) {
                        if beta > self.tree.current.borrow().data.alpha {
                            self.tree.current.borrow_mut().data.alpha = beta;
                            self.tree.current.borrow_mut().data.next_best = child_idx;
                            self.show_board_from_moves(&moves);
                        }
                    } else {
                        break;
                    }
                }
            }
        }

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
        info!("alpha: {}", self.tree.root.borrow().data.alpha);
        info!("beta: {}", self.tree.root.borrow().data.beta);
        info!("next_best: {:?}", self.tree.root.borrow().data.next_best);

        self.tree.goto_child(next_move_idx.unwrap());
        let next_move = self.tree.current.borrow().data.chess_move.unwrap();
        next_move
    }

    fn corrected_depth(&self, depth_correction: u8) -> u8 {
        self.tree.current.borrow().data.depth + depth_correction
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

    fn show_board_from_moves(&mut self, moves: &Vec<ChessMove>) {
        if moves.len() < 2{
            info!("---------------------------------");
            info!("moves: {}", moves_to_string(&moves));
            info!("next best moves:");
            let mut i = 0;
            loop {
                let next_best = self.tree.current.borrow().data.next_best;
                match next_best {
                    Some(best_idx) => {
                        i += 1;
                        self.tree.goto_child(best_idx);
                        let current = &self.tree.current.borrow().data;
                        info!(
                            "    move: {}, alpha: {}, beta: {}", 
                            chess_move_to_string(&current.chess_move.unwrap()), 
                            current.alpha, 
                            current.beta
                        );
                    },
                    None => {break;}
                }
            }
            for _ in 0..i {
                self.tree.goto_parent();
            }
            info!("depth: {}", moves.len());
            info!("index: {:?}", self.tree.current.borrow().index);
            info!("alpha: {}", self.tree.current.borrow().data.alpha);
            info!("beta: {}", self.tree.current.borrow().data.beta);
            show_board(board_from_moves(self.board, moves));
            info!("---------------------------------");
            info!("");
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

fn get_possible_moves(board: &Board) -> Vec<ChessMove>{
    let mut seed = board.get_hash();
    let mut legal_moves: Vec<ChessMove> = MoveGen::new_legal(&board).collect();
    for i in 0..legal_moves.len() {
        seed += legal_moves[i].get_source().to_int() as u64 * legal_moves[i].get_dest().to_int() as u64;
        let move_to = (seed % legal_moves.len() as u64) as usize;
        legal_moves.swap(i, move_to);
    }
    legal_moves
}


pub fn moves_to_string( moves: &Vec<ChessMove>) -> String{
    moves.iter().map(|mv| chess_move_to_string(mv))
    .fold(String::new(), |acc: String, e: String| acc + &e + ", ")
}

pub fn chess_move_to_string(mv: &ChessMove) -> String {
    format!("{}:{}", mv.get_source().to_string(), mv.get_dest().to_string())
}

pub fn assert_mv_eq(mv: &ChessMove, expcted: &str){
    assert_eq!(chess_move_to_string(mv), expcted);
}

pub fn assert_mv_ne(mv: &ChessMove, expcted: &str){
    assert_ne!(chess_move_to_string(mv), expcted);
}

#[cfg(test)]
mod tests {
    use chess::{CastleRights, BoardBuilder};

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
    fn test_avoiding_checkmate_in_one_white() {
        let textboard = r#"
        8|   |   |   |   | ♖ |   |   | ♔ |
        7|   |   |   |   |   |   |   |   |
        6|   |   |   |   |   |   |   |   |
        5|   |   |   |   |   |   |   |   |
        4|   | ♟︎ | ♟︎ |   |   |   |   | ♙ |
        3|   |   |   |   |   | ♙ | ♟︎ |   |
        2| ♟︎ |   |   |   |   | ♟︎ |   | ♟︎ |
        1|   |   |   |   |   |   |   | ♚ |
           a   b   c   d   e   f   g   h 
        "#;
        let board = board_from_textboard(
            textboard,
            CastleRights::NoRights,
            CastleRights::NoRights,
            Color::White,
        );
        let mut search = Search::new(&board, Color::White);
        println!("{:?}", search.run(2, None, None));
        let best = search.run(2, None, None);
        assert_mv_eq(&best, "h2:h3")
    }

    #[test]
    fn test_avoiding_checkmate_in_one_black() {
        let textboard = r#"
        8|   |   |   |   |   |   |   | ♔ |
        7|   |   | ♙ |   |   | ♙ |   | ♙ |
        6|   | ♙ |   |   |   | ♟︎ | ♙ |   |
        5| ♙ |   |   |   |   |   |   | ♟︎ |
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
        let best = search.run(2, None, None);
        assert_mv_eq(&best, "h7:h6")
    }

    #[test]
    fn test_avoiding_checkmate_in_two_white() {
        let textboard = r#"
        8|   |   |   |   | ♖ |   |   | ♔ |
        7|   |   |   |   | ♙ |   |   |   |
        6|   |   |   |   |   |   |   |   |
        5|   |   |   |   |   |   |   |   |
        4|   | ♟︎ | ♟︎ |   |   |   |   |   |
        3|   |   |   |   |   |   | ♟︎ | ♙ |
        2| ♟︎ |   |   |   |   | ♟︎ |   | ♟︎ |
        1|   |   |   |   |   |   |   | ♚ |
           a   b   c   d   e   f   g   h 
        "#;
        let board = board_from_textboard(
            textboard,
            CastleRights::NoRights,
            CastleRights::NoRights,
            Color::White,
        );
        let mut search = Search::new(&board, Color::White);
        let best = search.run(2, None, None);
        assert_eq!(best, search.run(3, None, None));
        assert_mv_eq(&best, "h1:g1")
    }

    #[test]
    fn test_avoiding_checkmate_in_two_black() {
        let textboard = r#"
        8|   |   |   |   |   |   |   | ♔ |
        7|   |   | ♙ |   |   | ♙ |   | ♙ |
        6|   | ♙ |   |   |   |   | ♙ | ♟︎ |
        5| ♙ |   |   |   |   |   |   |   |
        4|   |   |   |   |   |   |   |   |
        3|   |   |   |   |   |   |   |   |
        2|   |   |   |   | ♟︎ |   |   |   |
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

        let best = search.run(2, None, None);
        assert_eq!(best, search.run(3, None, None));
        assert_mv_eq(&best, "h8:g8")
    }


    #[test]
    fn test_real_situation_1() {
        let textboard = r#"
        8|   |   |   |   |   |   |   | ♔ |
        7|   |   | ♙ |   |   | ♙ |   | ♙ |
        6|   | ♙ |   |   |   |   | ♙ | ♟︎ |
        5| ♙ |   |   |   |   |   |   |   |
        4|   |   |   |   |   |   |   |   |
        3|   |   |   |   |   |   |   |   |
        2|   |   |   |   | ♟︎ |   |   |   |
        1|   |   |   |   | ♜ | ♚ |   |   |
           a   b   c   d   e   f   g   h 
        "#;
        let board = Board::from_str("2r2rk1/p1qnbppp/1p1ppn2/6N1/2PQ4/2N3P1/PP2PPKP/R1B2R2 w - - 3 14").unwrap();
        let mut search = Search::new(&board, Color::White);

        let best = search.run(3, None, None);
        assert_mv_ne(&best, "g2:h3")
    }
}
