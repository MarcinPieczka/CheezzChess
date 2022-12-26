use chess::{Board, ChessMove, Color, Game, MoveGen};
use log::info;
use std::str::FromStr;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, SyncSender};
use std::thread;
use std::thread::{JoinHandle};
use std::time::Duration;
use vampirc_uci::Duration as VampDuration;
use vampirc_uci::{UciMessage, UciTimeControl};

use crate::engine::lookup::Lookup;

pub mod lookup;

mod tests;

pub struct Engine {
    board: Option<Board>,
    best_move: Option<ChessMove>,
    channel_sender: SyncSender<UciMessage>,
    channel_receiver: Receiver<UciMessage>,
}

impl Default for Engine {
    fn default() -> Self {
        let (tx, rx) = mpsc::sync_channel(128);
        Engine {
            board: None,
            best_move: None,
            channel_sender: tx,
            channel_receiver: rx,
        }
    }
}

impl Engine {
    pub fn start(self) -> (JoinHandle<()>, SyncSender<UciMessage>) {
        let sender = self.channel_sender.clone();

        (thread::spawn(|| self.run()), sender)
    }

    fn run(mut self) {
        let timeout = Duration::from_millis(2);
        loop {
            if let Ok(message) = self.channel_receiver.recv_timeout(timeout) {
                if !self.handle_message(message) {
                    info!("Returning from event loop");
                    return;
                }
            }
        }
    }

    fn handle_message(&mut self, message: UciMessage) -> bool {
        info!("rx: {}", message);
        match message {
            UciMessage::Uci => {
                id();
                //option
                uciok();
            }
            UciMessage::Debug(_) => { /*ignore for now */ }
            UciMessage::IsReady => {
                readyok();
            }
            UciMessage::Register { later, name, code } => {}
            UciMessage::Position {
                startpos,
                fen,
                moves,
            } => {
                let mut game = if let Some(fen) = fen {
                    Game::from_str(fen.as_str()).unwrap()
                } else {
                    Game::new()
                };

                for mv in moves {
                    game.make_move(mv);
                }

                self.board = Some(game.current_position());
            }
            UciMessage::SetOption { .. } => {}
            UciMessage::UciNewGame => {
                //create a new game
                self.board = None;
            }
            UciMessage::Stop => match &self.board {
                None => {}
                Some(board) => {
                    let mut move_iter = MoveGen::new_legal(board);
                    match move_iter.next() {
                        Some(chess_move) => bestmove(chess_move, None),
                        None => {}
                    }
                }
            },

            UciMessage::PonderHit => {}
            UciMessage::Quit => {
                return false;
            }
            UciMessage::Go {
                time_control,
                search_control,
            } => {
                let move_time = calculate_time(time_control, self.board.unwrap().side_to_move());
                // sleep(move_time);
                let mut finder = Lookup::new(&self.board.unwrap());
                finder.run(100000);
                // 10000000 nodes is ok
            }
            _ => {}
        }

        true
    }
}


pub fn calculate_time(time_control: Option<UciTimeControl>, color: Color) -> Duration {
    let move_time;
    match time_control {
        Some(UciTimeControl::TimeLeft {
            white_time,
            black_time,
            white_increment,
            black_increment,
            moves_to_go,
        }) => {
            match color {
                Color::White => {
                    move_time = move_time_from_time_left(
                        white_time,
                        white_increment,
                        black_time,
                        black_increment,
                    );
                }
                Color::Black => {
                    move_time = move_time_from_time_left(
                        black_time,
                        black_increment,
                        white_time,
                        white_increment,
                    );
                }
            };
        }
        Some(UciTimeControl::MoveTime(value)) => {
            move_time = value;
        }
        _ => {
            move_time = VampDuration::seconds(1);
        }
    }
    Duration::from_millis(move_time.num_milliseconds() as u64)
}

fn move_time_from_time_left(
    my_time: Option<VampDuration>,
    my_increment: Option<VampDuration>,
    opponent_time: Option<VampDuration>,
    opponent_increment: Option<VampDuration>,
) -> VampDuration {
    // Returns a move time that will try to keep times left of the engine and the player
    // at a similar level
    let total_my_time = my_time.unwrap() + my_increment.unwrap_or(VampDuration::seconds(0));
    let total_opponent_time = opponent_time.unwrap_or(my_time.unwrap())
        + opponent_increment.unwrap_or(VampDuration::seconds(0));
    let time_ratio = total_opponent_time.num_milliseconds() as f64 / total_my_time.num_milliseconds() as f64;
    let move_time = my_time.unwrap().num_milliseconds() as f64 / (time_ratio.powf(5.0) * 40.0);
    VampDuration::milliseconds(move_time as i64) + my_increment.unwrap_or(VampDuration::seconds(0))
}

fn id() {
    reply(UciMessage::Id {
        name: Some("Loco-Chess".to_string()),
        author: None,
    });
    reply(UciMessage::Id {
        name: None,
        author: Some("Marcin Pieczka".to_string()),
    });
}

fn reply(message: UciMessage) {
    info!("tx: {:?}", message);
    println!("{}", message);
}

fn uciok() {
    reply(UciMessage::UciOk);
}

fn readyok() {
    reply(UciMessage::ReadyOk);
}

fn bestmove(best_move: ChessMove, ponder: Option<ChessMove>) {
    reply(UciMessage::BestMove { best_move, ponder });
}
