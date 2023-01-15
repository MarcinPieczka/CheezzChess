use crate::engine::eval::eval;
use chess::{Board, BoardStatus, CacheTable, ChessMove, Color, MoveGen, Piece, Square};
use log::info;
use vampirc_uci::UciMessage;

pub struct Position {
    chess_move: Option<ChessMove>,
    potential_next_moves: Vec<ChessMove>,
    parent_index: Option<usize>,
    best_next: Option<usize>,
    alpha: i16,
    beta: i16,
    depth: u8,
}

impl Position {
    pub fn new(
        chess_move: Option<ChessMove>,
        parent_index: Option<usize>,
        alpha: i16,
        beta: i16,
        depth: u8,
    ) -> Position {
        Position {
            chess_move: chess_move,
            potential_next_moves: vec![],
            parent_index: parent_index,
            best_next: None,
            alpha: alpha,
            beta: beta,
            depth: depth,
        }
    }
}

pub struct Search {
    positions: Vec<Position>,
    color: Color,
    board: Board,
}

impl Search {
    pub fn new(board: &Board, color: Color) -> Search {
        info!("Creating Search with color {:?}", board.side_to_move());
        let mut position = Position::new(None, None, i16::MIN, i16::MAX, 0);
        position.potential_next_moves = MoveGen::new_legal(board).collect();

        Search {
            positions: vec![position],
            color: color,
            board: board.clone(),
        }
    }

    pub fn run(&mut self, max_depth: u8, alpha: Option<i16>, beta: Option<i16>) {
        let mut i = 0;
        loop {
            let mut current = &mut self.positions[i];
            if current.depth < max_depth {
                match current.potential_next_moves.pop() {
                    Some(mv) => {
                        let mut new_position = Position::new(
                            Some(mv),
                            Some(i),
                            current.alpha,
                            current.beta,
                            current.depth + 1,
                        );
                        let new_board = &self.board_at_position(&new_position);
                        match new_board.status() {
                            BoardStatus::Ongoing => {
                                new_position.potential_next_moves =
                                    MoveGen::new_legal(new_board).collect();
                            }
                            _ => {
                                // here we need to eval the board
                            }
                        }

                        self.positions.push(new_position);
                    }
                    None => {
                        // Go to parent and set A/B
                        // if you're A or B "winns", then set best_next
                        // the current will also became the parrent again
                    }
                }
            } else {
                // eval all potential next moves and go up to parent
                // Go to parent and set A/B
                // if you're A or B "winns", then set best_next
            }
        }
    }

    pub fn board_at_position(&self, position: &Position) -> Board {
        let mut moves = vec![];
        let mut current_position = position;
        loop {
            match current_position.chess_move {
                Some(mv) => {
                    moves.push(mv);
                }
                None => {
                    break;
                }
            }

            match current_position.parent_index {
                Some(parent_index) => {
                    current_position = &self.positions[parent_index];
                }
                None => {
                    break;
                }
            }
        }
        let mut board = self.board.clone();

        for mv in moves.iter().rev() {
            board = board.make_move_new(*mv);
        }
        board
    }
}
