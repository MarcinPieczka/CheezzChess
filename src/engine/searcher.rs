use chess::{Board, ChessMove, Color, Game, MoveGen};
use trees::{fr, tr, Forest, Tree};

mod test;

#[derive(Debug, PartialEq)]
pub struct MoveNode {
    chess_move: ChessMove,
    evaluation: f32,
}

pub struct Searcher {
    moves: Tree<MoveNode>,
}

impl Searcher {
    pub fn build_tree(&mut self, board: Board) {
        let mut root = self.moves.root_mut();
        for chess_move in MoveGen::new_legal(&board) {
            root.append(-tr(MoveNode {
                chess_move,
                evaluation: 0.0,
            }));
        }
        // for mut node in self.moves.iter_mut() {
        //     println!("{:?}", MoveGen::new_legal(&board).next());
        //     for chess_move in MoveGen::new_legal(&board) {
        //         node.append(-tr(MoveNode{chess_move, evaluation: 0.0}));
        //     }
        // }
    }
}

impl Default for Searcher {
    fn default() -> Self {
        Searcher {
            moves: tr(MoveNode {
                chess_move: ChessMove::default(),
                evaluation: 0.0,
            }),
        }
    }
}
