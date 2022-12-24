use crate::engine::searcher::{MoveNode, Searcher};
use chess::{Board, ChessMove, Square};

#[test]
fn test_building_tree() {
    let mut searcher = Searcher::default();
    searcher.build_tree(Board::default());
    println!("{:?}", searcher.moves.iter().next().unwrap().data());
    assert_eq!(
        searcher.moves.iter().next().unwrap().data(),
        &MoveNode {
            chess_move: unsafe { ChessMove::new(Square::new(8), Square::new(16), None) },
            evaluation: 0.0
        }
    );
}
