use chess::{BitBoard, Board, BoardStatus, ChessMove, Color, Piece};

const CHECKMATE_EVAL: i16 = 10000;

const ROWS: (
    u64, u64, u64, u64, u64, u64, u64, u64, u64
) = (
    0, // just so that the index is correct
    255,
    255 << 8,
    255 << 16,
    255 << 24,
    255 << 32,
    255 << 40,
    255 << 48,
    255 << 56,
);

const BIG_CENTER: u64 = 0b0000000000000000001111000011110000111100001111000000000000000000;

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
            Color::White => Some(CHECKMATE_EVAL),
            Color::Black => Some(-CHECKMATE_EVAL),
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
            let mut pieces = (board.color_combined(color) & board.pieces(piece));
            let num_pieces = pieces.popcnt() as i16;
            if color == Color::Black {
                pieces = pieces.reverse_colors();
            }

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
                Piece::Pawn => score += multiplier * eval_pawns(pieces),
                _ => {}
            }
        }
    }
    score
}

fn eval_pawns(pawns: BitBoard) -> i16 {
    let mut score: u32 = 0;
    score += (pawns & BitBoard::new(ROWS.7)).popcnt() * 105;
    score += (pawns & BitBoard::new(ROWS.6)).popcnt() * 102;
    score += (pawns & BitBoard::new(ROWS.5 | ROWS.4 | ROWS.3 | ROWS.2)).popcnt() & 100;
    score as i16
}

#[test]
fn test_eval_pawns() {
    let score = eval_pawns(BitBoard::new(0b01010000 << 48));
    assert_eq!(score, 210);
}


// Test like this!!! 
// let A = r#"
// 8| ♜ | ♞ | ♝ | ♛ | ♚ | ♝ | ♞ | ♜ |
// 7| ♟︎ | ♟︎ | ♟︎ | ♟︎ | ♟︎ | ♟︎ | ♟︎ | ♟︎ |
// 6|   |   |   |   |   |   |   |   |
// 5|   |   |   |   |   |   |   |   |
// 4|   |   |   |   |   |   |   |   |
// 3|   |   |   |   |   |   |   |   |
// 2| ♙ | ♙ | ♙ | ♙ | ♙ | ♙ | ♙ | ♙ |
// 1| ♖ | ♘ | ♗ | ♕ | ♔ | ♗ | ♘ | ♖ |
//    a   b   c   d   e   f   g   h 
// "#;
// 
// such string as an input, and the tests would check
// if engine finds obvious next move, like, chooses 
// to check make, instead of taking quin