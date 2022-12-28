use chess::{Board, BoardStatus, ChessMove, Color, Piece};

const CHECKMATE_EVAL: i16 = 10000;

pub fn eval(start_board: &Board, moves: Vec<ChessMove>) -> i16 {
    let board = make_moves(start_board, moves);
    let score: i16;

    if board.status() == BoardStatus::Stalemate {
        return 0;
    }

    match eval_checkmate(&board) {
        Some(val) => score = val,
        None => {
            score = eval_material(&board);
        }
    }
    score
}

fn make_moves(start_board: &Board, moves: Vec<ChessMove>) -> Board {
    let mut board = start_board.clone();
    for mv in moves {
        board = board.make_move_new(mv);
    }
    return board;
}

fn eval_checkmate(board: &Board) -> Option<i16> {
    if board.status() == BoardStatus::Checkmate {
        match board.side_to_move() {
            Color::White => Some(-CHECKMATE_EVAL),
            Color::Black => Some(CHECKMATE_EVAL),
        }
    } else {
        None
    }
}

fn eval_material(board: &Board) -> i16 {
    let mut score: i16 = 0;
    for color in [Color::White, Color::Black] {
        for piece in [
            Piece::Queen,
            Piece::Rook,
            Piece::Bishop,
            Piece::Knight,
            Piece::Pawn,
        ] {
            let pieces = board.color_combined(color) & board.pieces(piece);
            let num_pieces = pieces.popcnt() as i16;

            let multiplier;
            match color {
                Color::White => multiplier = 1,
                Color::Black => multiplier = -1,
            }

            match piece {
                Piece::Queen => score += multiplier * 900 * num_pieces,
                Piece::Rook => score += multiplier * 500 * num_pieces,
                Piece::Bishop => score += multiplier * 310 * num_pieces,
                Piece::Knight => score += multiplier * 290 * num_pieces,
                Piece::Pawn => score += multiplier * 100 * num_pieces,
                _ => {}
            }
        }
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::utils::board_from_textboard;
    use chess::CastleRights;

    #[test]
    fn test_evaluating_pawns() {
        let textboard = r#"
        8|   |   |   |   | ♔ |   |   |   |
        7|   |   |   |   | ♙ |   |   |   |
        6|   |   |   |   |   |   |   |   |
        5|   |   |   |   |   |   |   |   |
        4|   |   |   |   |   |   |   |   |
        3|   |   |   |   |   |   |   |   |
        2|   |   |   |   | ♟︎ | ♟︎ |   |   |
        1|   |   |   |   | ♚ |   |   |   |
        a   b   c   d   e   f   g   h 
        "#;
        let board = board_from_textboard(
            textboard,
            CastleRights::NoRights,
            CastleRights::NoRights,
            Color::White,
        );
        assert_eq!(eval(&board, vec![]), 100);
    }

    #[test]
    fn test_evaluating_stalemate() {
        let textboard = r#"
        8|   |   |   |   |   |   |   | ♔ |
        7|   |   |   |   |   |   |   | ♟︎ |
        6|   |   |   |   |   |   |   | ♚ |
        5|   |   |   |   |   |   |   |   |
        4|   |   |   |   |   |   |   |   |
        3|   |   |   |   |   |   |   |   |
        2|   |   |   |   |   |   |   |   |
        1|   |   |   |   |   |   |   |   |
        a   b   c   d   e   f   g   h 
        "#;
        let board = board_from_textboard(
            textboard,
            CastleRights::NoRights,
            CastleRights::NoRights,
            Color::Black,
        );
        assert_eq!(board.status(), BoardStatus::Stalemate);
        assert_eq!(eval(&board, vec![]), 0);
    }

    #[test]
    fn test_evaluating_checkmate() {
        let textboard = r#"
        8|   |   |   |   |   |   |   | ♔ |
        7|   |   |   |   |   |   | ♟︎ | ♟︎ |
        6|   |   |   |   |   |   |   | ♚ |
        5|   |   |   |   |   |   |   |   |
        4|   |   |   |   |   |   |   |   |
        3|   |   |   |   |   |   |   |   |
        2|   |   |   |   |   |   |   |   |
        1|   |   |   |   |   |   |   |   |
        a   b   c   d   e   f   g   h 
        "#;
        let board = board_from_textboard(
            textboard,
            CastleRights::NoRights,
            CastleRights::NoRights,
            Color::Black,
        );
        assert_eq!(board.status(), BoardStatus::Checkmate);
        assert_eq!(eval(&board, vec![]), CHECKMATE_EVAL);
    }
}
