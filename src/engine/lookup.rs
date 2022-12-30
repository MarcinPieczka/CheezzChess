use crate::engine::eval::eval;
use chess::{Board, CacheTable, ChessMove, Color, MoveGen, Piece, Square};
use log::info;
use std::{
    collections::{HashMap, HashSet},
    os::unix::process::parent_id,
};
use vampirc_uci::UciMessage;

#[derive(Debug, PartialEq)]
pub struct Position {
    // all this should be compressed as much as possible later
    //
    // style_eval will be filled bottom up, and
    // position_eval top down
    last_move: ChessMove,
    eval: Option<i16>,
    depth: u8,
    parent: u64,
    best_next: Option<u32>,
}

impl Position {
    pub fn new(last_move: Option<ChessMove>, depth: u8, parent: u64) -> Position {
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
    cache: HashMap<u64, Vec<u32>>,
    cachable: u64,
}

impl Lookup {
    pub fn new(board: &Board) -> Lookup {
        info!("Creating Lookup with color {:?}", board.side_to_move());
        Lookup {
            positions: vec![Position::new(None, 0, 0)],
            color: board.side_to_move(),
            board: board.clone(),
            cache: HashMap::new(),
            cachable: 0,
        }
    }

    pub fn run(&mut self, max_nodes: usize) {
        self.find_positions(max_nodes, 4);
        self.min_max();
        self.best_move();
    }

    pub fn find_positions(&mut self, max_nodes: usize, max_depth: u8) {
        let mut i: usize = 0;
        info!("Looking for up to {:?} nodes", max_nodes);
        info!("Start position eval: {:?}", eval(&self.board, vec![]));
        loop {
            let children: Vec<Position>;
            {
                let parent = &self.positions[i];
                if parent.depth == max_depth {
                    info!("Reached max depth: {}", parent.depth);
                    children = vec![];
                    break;
                }

                let parent_board = self.get_board(parent);
                let parent_hash = parent_board.get_hash();
                self.cachable += 1;
                if self.cache.contains_key(&parent_hash) {
                    self.cache
                        .entry(parent_hash)
                        .and_modify(|parents| parents.push(i as u32));
                    children = vec![];
                } else {
                    self.cache.insert(parent_hash, vec![i as u32]);
                    children = MoveGen::new_legal(&parent_board)
                        .filter(|mv| parent_board.legal(*mv))
                        .map(|mv| Position::new(Some(mv), self.positions[i].depth + 1, parent_hash))
                        .collect();
                }
            }
            self.positions.extend(children);

            i += 1;
            if self.positions.len() >= max_nodes {
                break;
            }
        }
        info!(
            "number of positions after search: {}, cachable: {} cached: {}",
            self.positions.len(),
            self.cachable,
            self.cache.len()
        );
    }

    pub fn all_moves(&self, position: &Position) -> Vec<ChessMove> {
        let mut moves = vec![];
        let mut parent = position.clone();
        loop {
            if parent.depth == 0 {
                break;
            }
            moves.push(parent.last_move);
            parent = &self.positions[self.cache.get(&parent.parent).unwrap()[0] as usize];
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
            if self.positions[i].eval.is_none() {
                self.positions[i].eval = Some(self.eval_position(&self.positions[i]));
            }
            for parent_i_32 in self.cache.get(&self.positions[i].parent).unwrap().iter() {
                let parent_i = parent_i_32.clone() as usize;
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
            }
        }
    }

    pub fn opti_min_max(&mut self) {
        info!("Starting min max with {} positions", self.positions.len());
        let mut best_eval: Option<i16> = None;
        let mut best_next: Option<u32> = None;
        let mut current_parent: Option<u64> = None;
        for i in (1..(self.positions.len())).rev() {
            if self.positions[i].eval.is_none() {
                self.positions[i].eval = Some(self.eval_position(&self.positions[i]));
            }
            if current_parent.is_some() && current_parent != Some(self.positions[i - 1].parent) {
                for parent_i_32 in self.cache.get(&self.positions[i].parent).unwrap().iter() {
                    let parent_i = parent_i_32.clone() as usize;
                    self.positions[parent_i].eval = best_eval;
                    self.positions[parent_i].best_next = best_next;
                    best_eval = None;
                    best_next = None;
                    current_parent = None;
                }
            } else {
                current_parent = Some(self.positions[i].parent);
                if best_eval.is_none() {
                    best_eval = self.positions[i].eval;
                    best_next = Some(i as u32);
                } else {
                    match (self.positions[i].depth % 2, self.color) {
                        (1, Color::White) | (0, Color::Black) => {
                            if self.positions[i].eval.unwrap() > best_eval.unwrap() {
                                best_eval = self.positions[i].eval;
                                best_next = Some(i as u32);
                            }
                        }
                        (0, Color::White) | (1, Color::Black) => {
                            if self.positions[i].eval.unwrap() < best_eval.unwrap() {
                                best_eval = self.positions[i].eval;
                                best_next = Some(i as u32);
                            }
                        }
                        _ => {
                            panic!()
                        }
                    }
                }
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
        info!(
            "Best move: {} has eval: {:?}, next moves:",
            best.last_move, best.eval
        );
        let mut parent = &self.positions[best.best_next.unwrap() as usize];
        loop {
            info!("move: {}, depth: {}", parent.last_move, parent.depth);
            match parent.best_next {
                Some(next_i) => {
                    parent = &self.positions[next_i as usize];
                }
                None => {
                    break;
                }
            }
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

#[cfg(test)]
mod tests {
    use crate::engine::lookup::{Lookup, Position};
    use chess::{ChessMove, Game, Square};

    #[test]
    fn test_generating_positions_generates_correct_first_move() {
        let board = Game::new().current_position();
        let mut lookup = Lookup::new(&board);
        println!("{:?}", board.side_to_move());
        lookup.find_positions(5, 6);
        let expected_move = unsafe { ChessMove::new(Square::new(8), Square::new(16), None) };
        println!("{:?}", lookup.positions);
        assert_eq!(
            lookup.positions[1],
            Position::new(Some(expected_move), 1, 0)
        );
    }
}
