use crate::engine::searcher::{MoveNode, Searcher};
use chess::{Board, ChessMove, Game, Square};

#[test]
fn test_building_tree() {
    let board = Game::new().current_position();
    let mut searcher = Searcher::new(&board);
    println!("{:?}", board.side_to_move());
    searcher.build_tree();
    let expected_move = unsafe { ChessMove::new(Square::new(8), Square::new(16), None) };
    let expected_board = Game::new().current_position();
    assert_eq!(
        searcher.moves.iter().next().unwrap().data(),
        &MoveNode::new(&expected_board, Some(expected_move))
    );
}
