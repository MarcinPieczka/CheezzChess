use trees::{tr, Tree, fr, Forest};
use chess::{Board, ChessMove, Color, Game, MoveGen};


struct MoveNode {
    chess_move: ChessMove,
    evaluation: f32,
}

pub struct Searcher {
    moves: Tree<MoveNode>
}

impl Searcher {
    fn build_tree(&mut self, board: Board) {
        for mut node in self.moves.iter_mut() {
            for chess_move in MoveGen::new_legal(&board) {
                node.append(-tr(MoveNode{chess_move, evaluation: 0.0}));
            }
        }
    }
}