use chess::{Board, BoardBuilder, CastleRights, Color, File, Piece, Rank, Square};
use std::string::String as StdString;
use string::String;


#[cfg(not(test))] 
use log::{info, warn};
 
#[cfg(test)]
use std::{println as info, println as warn};


pub fn show_board(board: Board) {
    for l in (0..8).rev() {
        let mut line = "".to_string();
        for f in 0..8 {
            let square = unsafe { Square::new(f + l * 8) };
            match (board.piece_on(square), board.color_on(square)) {
                (Some(piece), Some(color)) => {
                    line += &format!("| {} ", piece_to_char(piece, color));
                },
                _ => line += "|   ",
            }
        }
        info!("{}", line);
    }
}


pub fn board_from_textboard(
    textboard: &str,
    white_castle_rights: CastleRights,
    black_castle_rights: CastleRights,
    side_to_move: Color,
) -> Board {
    let mut position = BoardBuilder::new();
    let lines = textboard_lines(textboard);
    for i in 0..8 {
        let rank = 7 - i;
        let pieces: Vec<Option<(Piece, Color)>> = lines[i]
            .split("|")
            .filter(|field| field.len() > 1)
            .map(|line| char_to_piece(line.trim()))
            .collect();
        for (file, piece_color) in pieces.iter().enumerate() {
            match piece_color {
                Some((piece, color)) => {
                    position.piece(
                        Square::make_square(Rank::from_index(rank), File::from_index(file)),
                        *piece,
                        *color,
                    );
                }
                None => {}
            }
        }
    }
    position.castle_rights(Color::White, white_castle_rights);
    position.castle_rights(Color::Black, black_castle_rights);
    position.side_to_move(side_to_move);

    Board::try_from(position).unwrap()
}

fn textboard_lines(textboard: &str) -> Vec<String> {
    let text: String = String::from_str(textboard);
    text.split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| String::from_str(line))
        .collect()
}

fn piece_to_char(piece: Piece, color: Color) -> StdString {
    match (piece, color) {
        (Piece::King, Color::White) => "♚".to_string(),
        (Piece::Queen, Color::White) => "♛".to_string(),
        (Piece::Rook, Color::White) => "♜".to_string(),
        (Piece::Bishop, Color::White) => "♝".to_string(),
        (Piece::Knight, Color::White) => "♞".to_string(),
        (Piece::Pawn, Color::White) => "♟︎".to_string(),
        (Piece::King, Color::Black) => "♔".to_string(),
        (Piece::Queen, Color::Black) => "♕".to_string(),
        (Piece::Rook, Color::Black) => "♖".to_string(),
        (Piece::Bishop, Color::Black) => "♗".to_string(),
        (Piece::Knight, Color::Black) => "♘".to_string(),
        (Piece::Pawn, Color::Black) => "♙".to_string(),
    }
}

fn char_to_piece(char: &str) -> Option<(Piece, Color)> {
    match char {
        "♚" => Some((Piece::King, Color::White)),
        "♛" => Some((Piece::Queen, Color::White)),
        "♜" => Some((Piece::Rook, Color::White)),
        "♝" => Some((Piece::Bishop, Color::White)),
        "♞" => Some((Piece::Knight, Color::White)),
        "♟︎" => Some((Piece::Pawn, Color::White)),
        "♔" => Some((Piece::King, Color::Black)),
        "♕" => Some((Piece::Queen, Color::Black)),
        "♖" => Some((Piece::Rook, Color::Black)),
        "♗" => Some((Piece::Bishop, Color::Black)),
        "♘" => Some((Piece::Knight, Color::Black)),
        "♙" => Some((Piece::Pawn, Color::Black)),
        _ => None,
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generating_start_position() {
        let position = r#"
        8| ♖ | ♘ | ♗ | ♕ | ♔ | ♗ | ♘ | ♖ |
        7| ♙ | ♙ | ♙ | ♙ | ♙ | ♙ | ♙ | ♙ |
        6|   |   |   |   |   |   |   |   |
        5|   |   |   |   |   |   |   |   |
        4|   |   |   |   |   |   |   |   |
        3|   |   |   |   |   |   |   |   |
        2| ♟︎ | ♟︎ | ♟︎ | ♟︎ | ♟︎ | ♟︎ | ♟︎ | ♟︎ |
        1| ♜ | ♞ | ♝ | ♛ | ♚ | ♝ | ♞ | ♜ |
           a   b   c   d   e   f   g   h 
        "#;
        let board = board_from_textboard(
            position,
            CastleRights::Both,
            CastleRights::Both,
            Color::White,
        );
        assert_eq!(board, Board::default());
    }
}
