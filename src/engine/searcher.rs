use chess::{BitBoard, Board, ChessMove, Color, Game, MoveGen};
use std::mem;
use trees::{fr, tr, Forest, Tree};

mod test;

#[derive(Debug, PartialEq)]
pub struct MoveNode {
    last_move: ChessMove,
    board: Box<Option<Board>>,
    position_eval: f32,
    style_eval: f32,
    minmax_multi: i8,
}

impl MoveNode {
    pub fn new(board: &Board, last_move: Option<ChessMove>) -> MoveNode {
        match last_move {
            Some(m) => MoveNode {
                last_move: m,
                board: Box::new(Some(board.make_move_new(m))),
                position_eval: 0.0,
                style_eval: 0.0,
                minmax_multi: -1,
            },
            None => MoveNode {
                last_move: ChessMove::default(),
                board: Box::new(Some(board.clone())),
                position_eval: 0.0,
                style_eval: 0.0,
                minmax_multi: -1,
            },
        }
    }
}

pub struct Searcher {
    moves: Tree<MoveNode>,
}

impl Searcher {
    pub fn new(board: &Board) -> Searcher {
        Searcher {
            moves: tr(MoveNode::new(&board, None)),
        }
    }
    pub fn build_tree(&mut self) {
        let mut nodes: u32 = 0;
        let mut depth: u8 = 1;

        let mut root = self.moves.root_mut();
        let board = root.data().board.unwrap();
        println!("{:?}", board.side_to_move());

        for chess_move in MoveGen::new_legal(&board) {
            println!("{:?}", chess_move);
            root.append(-tr(MoveNode::new(&board, Some(chess_move))));
            nodes += 1;
        }
        // for d in 1..5 {
        //     for mut node in self.moves.iter_mut() {
        //         for chess_move in MoveGen::new_legal(&board) {
        //             node.append(-tr(MoveNode{chess_move, position_eval: 0.0}));
        //         }
        //     }
        // }
    }
}
