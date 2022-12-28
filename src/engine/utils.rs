use chess::{Piece, Board, Color};
use string::String;
use std::string::String as StdString;


pub fn board_from_textboard(textboard: &str)-> Board {
    let lines = preprocess_textboard(textboard);
    
    Board::default()
}

fn preprocess_textboard(textboard: &str)-> Vec<String> {
    let text: String = String::from_str(textboard);
    text.split("\n")
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .map(|line| String::from_str(line))
        .collect()
}

fn piece_to_char(piece: Piece, color: Color)-> StdString {
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
        (Piece::Pawn, Color::Black) => "♙".to_string()
    }
}

fn char_to_piece(char: &str)-> Option<(Piece, Color)> {
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
        _ => None
    }
}


#[cfg(test)]
mod tests {
    use super::board_from_str;


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
        let board = board_from_str(position);
        assert_eq!(board, Board::default());
    }
}