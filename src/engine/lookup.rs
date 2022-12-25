use chess::{BitBoard, Board, ChessMove, Color, Game, MoveGen};
use std::mem;
use trees::{fr, tr, Forest, Tree};

mod test;

#[derive(Debug)]
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
    parent: u32
}

impl Position {
    pub fn new(board: &Board, last_move: Option<ChessMove>, depth: u8, parent: u32)-> Position {
        match last_move {
            Some(mv) => Position {
                last_move: mv,
                board: Some(Box::new(board.make_move_new(mv))),
                position_eval: 0.0,
                style_eval: 0.0,
                depth: depth,
                parent: parent
            },
            None => Position {
                last_move: ChessMove::default(),
                board: Some(Box::new(board.clone())),
                position_eval: 0.0,
                style_eval: 0.0,
                depth: depth,
                parent: parent
            },
        }
    }
}


pub struct Lookup {
    positions: Vec<Position>
}

impl Lookup {
    pub fn find_positions(&mut self, board: &Board, max_nodes: u32) {
        let mut i: u32 = 0;
        self.positions.push(Position::new(board, None, 0, 0));
        loop {
            let children: Vec<Position>;
            {
                let parent = &self.positions[i as usize];

                children = MoveGen::new_legal(
                    &(*parent.board.as_ref().unwrap())
                ).map(|mv| Position::new(
                    &(*parent.board.as_ref().unwrap()), 
                    Some(mv), 
                    self.positions[i as usize].depth + 1, 
                    i)
                ).collect();
            }

            self.positions.extend(children);

            self.positions[i as usize].board = None;
            i += 1;
            if i == max_nodes {
                break;
            }
        };
    }
}