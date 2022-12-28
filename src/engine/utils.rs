use chess::Board;

pub fn board_from_str(board_str: &str)-> Board {
    Board::default()
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