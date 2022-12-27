use crate::engine::eval::eval;
use chess::{Board, ChessMove, Color, MoveGen, Piece, Square};
use log::info;
use vampirc_uci::UciMessage;

mod test;

#[derive(Debug, PartialEq)]
pub struct Position {
    // all this should be compressed as much as possible later
    //
    // style_eval will be filled bottom up, and
    // position_eval top down
    last_move: ChessMove,
    eval: Option<i16>,
    depth: u8,
    parent: u32,
    best_next: Option<u32>,
}

// let const A = r#"
// 8| ♜ | ♞ | ♝ | ♛ | ♚ | ♝ | ♞ | ♜ |
// 7| ♟︎ | ♟︎ | ♟︎ | ♟︎ | ♟︎ | ♟︎ | ♟︎ | ♟︎ |
// 6|   |   |   |   |   |   |   |   |
// 5|   |   |   |   |   |   |   |   |
// 4|   |   |   |   |   |   |   |   |
// 3|   |   |   |   |   |   |   |   |
// 2| ♙ | ♙ | ♙ | ♙ | ♙ | ♙ | ♙ | ♙ |
// 1| ♖ | ♘ | ♗ | ♕ | ♔ | ♗ | ♘ | ♖ |
//    a   b   c   d   e   f   g   h 
// "#;

impl Position {
    pub fn new(last_move: Option<ChessMove>, depth: u8, parent: u32) -> Position {
        match last_move {
            Some(mv) => Position {
                last_move: mv,
                best_next: None,
                eval: None,
                depth: depth,
                parent: parent,
            },
            None => Position {
                last_move: ChessMove::default(),
                best_next: None,
                eval: None,
                depth: depth,
                parent: parent,
            },
        }
    }
}

pub struct Lookup {
    positions: Vec<Position>,
    color: Color,
    board: Board,
}

impl Lookup {
    pub fn new(board: &Board) -> Lookup {
        info!("Creating Lookup with color {:?}", board.side_to_move());
        Lookup {
            positions: vec![Position::new(None, 0, 0)],
            color: board.side_to_move(),
            board: board.clone(),
        }
    }

    pub fn run(&mut self, max_nodes: usize) {
        self.find_positions(max_nodes);
        self.min_max();
        self.best_move();
    }

    pub fn find_positions(&mut self, max_nodes: usize) {
        let mut i: usize = 0;
        info!("Looking for up to {:?} nodes", max_nodes);
        info!("Start position eval: {:?}", eval(&self.board, vec![]));
        loop {
            let children: Vec<Position>;
            {
                let parent = &self.positions[i];
                let parent_board = self.get_board(parent);

                children = MoveGen::new_legal(&parent_board)
                    .filter(|mv| parent_board.legal(*mv))
                    .map(|mv| Position::new(Some(mv), self.positions[i].depth + 1, i as u32))
                    .collect();
            }
            self.positions.extend(children);

            i += 1;
            if self.positions.len() >= max_nodes {
                break;
            }
        }
        info!("number of positions after search: {}", self.positions.len());
    }

    pub fn all_moves(&self, position: &Position) -> Vec<ChessMove> {
        let mut moves = vec![];
        if position.depth == 0 {
            return moves;
        }
        let mut parent = position.clone();
        loop {
            moves.push(parent.last_move);
            if parent.parent == 0 {
                break;
            }
            parent = &self.positions[parent.parent as usize];
        }
        moves.reverse();
        moves
    }

    pub fn get_board(&self, position: &Position) -> Board {
        let mut board = self.board.clone();
        for mv in self.all_moves(position) {
            board = board.make_move_new(mv);
        }
        board
    }

    pub fn min_max(&mut self) {
        info!("Starting min max with {} positions", self.positions.len());
        for i in (1..(self.positions.len())).rev() {
            let parent_i = self.positions[i].parent as usize;

            if self.positions[i].eval.is_none() {
                self.positions[i].eval = Some(self.eval_position(&self.positions[i]));
            }

            match self.positions[parent_i].eval {
                None => {
                    self.positions[parent_i].eval = self.positions[i].eval;
                    self.positions[parent_i].best_next = Some(i as u32);
                }
                Some(parent_eval) => match (self.positions[i].depth % 2, self.color) {
                    (1, Color::White) | (0, Color::Black) => {
                        if self.positions[i].eval.unwrap() > parent_eval {
                            self.positions[parent_i].eval = self.positions[i].eval;
                            self.positions[parent_i].best_next = Some(i as u32);
                        }
                    }
                    (0, Color::White) | (1, Color::Black) => {
                        if self.positions[i].eval.unwrap() < parent_eval {
                            self.positions[parent_i].eval = self.positions[i].eval;
                            self.positions[parent_i].best_next = Some(i as u32);
                        }
                    }
                    _ => {
                        panic!()
                    }
                },
            }

            if i == self.positions.len() - 1 {
                info!(
                    "Child eval: {:?}, Parent eval: {:?}, Depth: {}",
                    self.positions[i].eval, self.positions[parent_i].eval, self.positions[i].depth
                );
                show_board(self.get_board(&self.positions[i]));
            }
        }
    }

    pub fn eval_position(&self, position: &Position) -> i16 {
        eval(&self.board, self.all_moves(position))
    }

    pub fn best_move(&self) {
        info!("Best eval {:?}", self.positions[0].eval);
        let best_i = self.positions[0].best_next.unwrap() as usize;
        let best = &self.positions[best_i];

        println!(
            "{}",
            UciMessage::BestMove {
                best_move: best.last_move,
                ponder: None
            }
        );

        let mut iterator = self.positions.iter();
        iterator.next();
        for position in iterator {
            if position.depth > 1 {
                break;
            }
            info!("Move: {} has eval: {:?}", position.last_move, position.eval);
        }
        info!("Best move: {} has eval: {:?}, next moves:", best.last_move, best.eval);
        let mut parent = &self.positions[best.best_next.unwrap() as usize];
        loop {
            info!("{}", parent.last_move);
            match parent.best_next {
                Some(next_i) => {
                    parent = &self.positions[next_i as usize];
                }, None => {break;}  
            }
        }
        for position in self.positions.iter() {
            if position.depth > 2 {
                break;
            } if position.depth < 2 {
                continue;
            } if position.depth < 2 {
                continue;
            }
            info!("Move: {} has eval: {:?}", position.last_move, position.eval);
        }

    }
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
