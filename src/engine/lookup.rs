use chess::{BitBoard, Board, ChessMove, Color, File, Game, MoveGen, Piece, Rank, Square};
use enum_iterator::all;
use std::mem;
use trees::{fr, tr, Forest, Tree};

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
        match last_move {
            Some(mv) => Position {
                last_move: mv,
                board: Some(Box::new(board.make_move_new(mv))),
                position_eval: 0.0,
                style_eval: 0.0,
                depth: depth,
                parent: parent,
            },
            None => Position {
                last_move: ChessMove::default(),
                board: Some(Box::new(board.clone())),
                position_eval: 0.0,
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

    pub fn find_positions(&mut self, max_nodes: usize) {
        let mut i: usize = 0;
        loop {
            let children: Vec<Position>;
            {
                let parent = &self.positions[i];

                children = MoveGen::new_legal(&(*parent.board.as_ref().unwrap()))
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
    }

    pub fn evaluate_leafs(&mut self) {
        let last_parent = self.positions.last().unwrap().parent.clone();
        for i in (self.positions.len() - 1)..(last_parent as usize) {
            self.positions[i].position_eval = self.eval_position(&self.positions[i]);
        }
    }

    pub fn eval_position(&self, position: &Position) -> f32 {
        let board = &(*position.board.as_ref().unwrap());
        let mut score: f32 = 0.0;
        for color in [Color::White, Color::Black] {
            for piece in [
                Piece::Queen,
                Piece::Rook,
                Piece::Bishop,
                Piece::Knight,
                Piece::Pawn,
            ] {
                let num_pieces = (board.color_combined(color) & board.pieces(piece)).popcnt();
                let mut multiplier = 1.0;
                match color {
                    _ if color == self.color => {}
                    _ => multiplier = -1.0,
                }

                match piece {
                    Piece::Queen => score += multiplier * 9.0,
                    Piece::Rook => score += multiplier * 5.0,
                    Piece::Bishop => score += multiplier * 3.1,
                    Piece::Knight => score += multiplier * 2.9,
                    Piece::Pawn => score += multiplier * 1.0,
                    _ => {}
                }
            }
        }
        score
    }

    pub fn min_max(&mut self) {
        for i in (self.positions.len() - 1)..0 {
            let parent_i = self.positions[i].parent as usize;
            let mut values = vec![
                self.positions[parent_i].position_eval,
                self.positions[i].position_eval,
            ];
            values.sort_by(|a, b| a.total_cmp(&b));
            self.positions[parent_i].position_eval =
                values[(self.positions[i].depth % 2 - 1) as usize];
        }
    }
}
