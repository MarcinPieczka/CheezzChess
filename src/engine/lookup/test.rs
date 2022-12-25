use crate::engine::lookup::{Lookup, Position};
use chess::{Board, ChessMove, Game, Square};

#[test]
fn test_generating_positions() {
    let board = Game::new().current_position();
    let mut lookup = Lookup::new(&board);
    println!("{:?}", board.side_to_move());
    lookup.find_positions(5);
    let expected_move = unsafe { ChessMove::new(Square::new(8), Square::new(16), None) };
    println!("{:?}", lookup.positions);
    assert_eq!(
        lookup.positions[1],
        Position::new(&board, Some(expected_move), 1, 0)
    );
}
