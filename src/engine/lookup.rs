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
    board: Option<Box<Board>>,
    position_eval: f32,
    style_eval: f32,
    depth: u8,
    parent: u32,
}

impl Position {
    pub fn new(board: &Board, last_move: Option<ChessMove>, depth: u8, parent: u32) -> Position {
        let eval: f32;
        match depth % 2 {
            1 => {eval = 99999999.0;},
            _ => {eval = -99999999.0;},
        }
        match last_move {
            Some(mv) => Position {
                last_move: mv,
                board: Some(Box::new(board.make_move_new(mv))),
                position_eval: eval,
                style_eval: 0.0,
                depth: depth,
                parent: parent,
            },
            None => Position {
                last_move: ChessMove::default(),
                board: Some(Box::new(board.clone())),
                position_eval: eval,
                style_eval: 0.0,
                depth: depth,
                parent: parent,
            },
        }
    }
}

pub struct Lookup {
    positions: Vec<Position>,
    color: Color,
}

impl Lookup {
    pub fn new(board: &Board) -> Lookup {
        Lookup {
            positions: vec![Position::new(board, None, 0, 0)],
            color: board.side_to_move(),
        }
    }

    pub fn run(&mut self, max_nodes: usize) {
        self.find_positions(max_nodes);
        self.evaluate_leafs();
        self.min_max();
        self.best_move();
    }

    pub fn find_positions(&mut self, max_nodes: usize) {
        let mut i: usize = 0;
        info!("looking for up to {:?} nodes", max_nodes);
        loop {
            let children: Vec<Position>;
            {
                let parent = &self.positions[i];
                
                children = MoveGen::new_legal(&(*parent.board.as_ref().unwrap()))
                    .filter(|mv| {parent.board.as_ref().unwrap().legal(*mv)})
                    .map(|mv| {
                        Position::new(
                            &(*parent.board.as_ref().unwrap()),
                            Some(mv),
                            self.positions[i].depth + 1,
                            i as u32,
                        )
                    })
                    .collect();
            }

            self.positions.extend(children);

            self.positions[i].board = None;
            i += 1;
            if self.positions.len() >= max_nodes {
                break;
            }
        }
        info!("number of positions after search: {}", self.positions.len());
    }

    pub fn evaluate_leafs(&mut self) {
        let last_parent = self.positions.last().unwrap().parent.clone();
        for i in (((last_parent + 1) as usize)..(self.positions.len())).rev() {
            self.positions[i].position_eval = self.eval_position(&self.positions[i]);
        }
    }

    pub fn eval_position(&self, position: &Position) -> f32 {
        let board = &(*position.board.as_ref().unwrap());
        let mut score: f32 = 0.0;
        for color in [Color::White, Color::Black] {
            let mut color_score: f32 = 0.0;
            for piece in [
                Piece::Queen,
                Piece::Rook,
                Piece::Bishop,
                Piece::Knight,
                Piece::Pawn,
            ] {
                let mut piece_score: f32 = 0.0;

                let num_pieces = (board.color_combined(color) & board.pieces(piece)).popcnt();
                let mut multiplier = 1.0;
                match color {
                    _ if color == self.color => {}
                    _ => multiplier = -1.0,
                }

                match piece {
                    Piece::Queen => piece_score += multiplier * 9.0,
                    Piece::Rook => piece_score += multiplier * 5.0,
                    Piece::Bishop => piece_score += multiplier * 3.1,
                    Piece::Knight => piece_score += multiplier * 2.9,
                    Piece::Pawn => piece_score += multiplier * 1.0,
                    _ => {}
                }
                color_score += piece_score * num_pieces as f32;
            }
            score += color_score;
        }
        score
    }

    pub fn min_max(&mut self) {
        info!("Starting min max with {} positions", self.positions.len());
        for i in (1..(self.positions.len())).rev() {
            let parent_i = self.positions[i].parent as usize;
            let mut values = vec![
                self.positions[i].position_eval,
                self.positions[parent_i].position_eval,
            ];
            values.sort_by(|a, b| a.total_cmp(&b));
            self.positions[parent_i].position_eval =
                values[(self.positions[i].depth % 2) as usize];
            if i > self.positions.len() - 5 {
                info!("Child eval: {}, Parent eval: {}, Depth: {}", 
                self.positions[i].position_eval, 
                self.positions[parent_i].position_eval,
                self.positions[i].depth
            );
                show_board(self.positions[i].board.as_ref().unwrap().clone().as_ref().clone());
                for piece in *(*self.positions[i].board.as_ref().unwrap()).pieces(Piece::Pawn) {
                    info!("{}", piece);
                }
                info!("{}", &(*self.positions[i].board.as_ref().unwrap()).to_string());
            }
        }
    }

    pub fn best_move(&self) {
        let mut best = ChessMove::default();
        let mut max_eval = -999999990.0;
        let mut iterator = self.positions.iter();
        iterator.next();
        for position in iterator {
            info!("Move: {} has eval: {}", position.last_move, position.position_eval);
            if position.position_eval > max_eval {
                max_eval = position.position_eval;
                best = position.last_move;
            }
            if position.depth > 1 {
                break;
            }
        }
        println!("{}", UciMessage::BestMove { best_move: best, ponder: None });
    }
}



pub fn show_board(board: Board) {
    for l in 0..8 {
        let mut line = "".to_string();
        for f in 0..8 {
            match board.piece_on(unsafe{Square::new(f + l * 8)}) {
                Some(Piece::King) => line += "|K ",
                Some(Piece::Queen) => line += "|Q ",
                Some(Piece::Rook) => line += "|R ",
                Some(Piece::Bishop) => line += "|B ",
                Some(Piece::Knight) => line += "|Kn",
                Some(Piece::Pawn) => line += "|p ",
                None => line += "|  "
            }
        }
        info!("{}", line);
    }
}
