
use chess::{Board, ChessMove, Color, MoveGen};
use log::info;
use trees::{tr, Tree};


pub struct Position {
    chess_move: ChessMove,
    potential_next_moves: Option<Vec<ChessMove>>,
    alpha: i16,
    beta: i16,
    depth: u8,
}

impl Position {
    pub fn new(chess_move: ChessMove, alpha: i16, beta: i16, depth: u8) -> Position {
        Position {
            chess_move: chess_move,
            potential_next_moves: None,
            alpha: alpha,
            beta: beta,
            depth: depth,
        }
    }
}

pub struct Search {
    positions: Tree<Position>,
    color: Color,
    board: Board,
    chess_move: ChessMove,
}

impl Search {
    pub fn new(board: &Board, color: Color, chess_move: ChessMove) -> Search {
        info!("Creating Search with color {:?}", board.side_to_move());
        Search {
            positions: tr(Position::new(chess_move, i16::MIN, i16::MAX, 0)),
            color: color,
            board: board.clone(),
            chess_move: chess_move,
        }
    }

    pub fn run(&mut self, max_depth: u8, alpha: Option<i16>, _beta: Option<i16>) {
        let mut root = self.positions.root_mut();
        root.data_mut().alpha = alpha.unwrap_or(root.data_mut().alpha);
        root.data_mut().beta = alpha.unwrap_or(root.data_mut().beta);

        let mut moves = vec![root.data().chess_move];
        let mut current = self.positions.root_mut();
        loop {
            if current.data().depth < max_depth {
                if current.data().potential_next_moves.is_none() {
                    let board = board_from_moves(self.board.clone(), &moves);
                    let legal_moves = MoveGen::new_legal(&board).collect();
                    current.data_mut().potential_next_moves = Some(legal_moves);
                }
                match &mut current.data_mut().potential_next_moves {
                    Some(potential_moves) if potential_moves == &[] => {
                        if current.has_no_child() {}
                        // There are no more moves either because we used them all
                        // or there vere none to begin with.
                        //
                        // If there are children, then the a/b are set correctly
                        // and we can move up with the a/b
                        //
                        // If there were no potential moves, then we have to evaluate this position
                    }
                    Some(potential_moves) => {
                        let next_move = potential_moves.pop().unwrap();
                        let _alpha = current.data().alpha;
                        let _beta = current.data().beta;
                        let _depth = current.data().depth + 1;

                        //current.push_back(tr(Position::new(next_move, alpha, beta, depth)));
                        //current = current.back_mut().unwrap().back_mut().unwrap();

                        moves.push(next_move);
                    }
                    None => {
                        panic!("this field should have been initialized already")
                    }
                }
            }
        }
    }
}

fn board_from_moves(initial_board: Board, moves: &Vec<ChessMove>) -> Board {
    let mut board = initial_board.clone();
    for mv in moves.iter() {
        board = board.make_move_new(*mv);
    }
    board
}
