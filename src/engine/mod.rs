use chess::{Board, ChessMove, Color, Game, MoveGen};
use std::str::FromStr;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, SyncSender};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use vampirc_uci::{UciInfoAttribute, UciMessage, UciTimeControl};
use log::info;


struct SearchHandle {
    start_time: Instant,
    search_length: Option<Duration>,
}

pub struct Engine {
    board: Option<Board>,
    best_move: Option<ChessMove>,
    channel_sender: SyncSender<UciMessage>,
    channel_receiver: Receiver<UciMessage>,
    searcher: Option<SearchHandle>,
}

impl SearchHandle {
    fn new(search_length: Option<Duration>) -> Self {
        let start_time = Instant::now();

        SearchHandle {
            search_length,
            start_time,
        }
    }

    fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }
}

impl Default for Engine {
    fn default() -> Self {
        let (tx, rx) = mpsc::sync_channel(128);
        Engine {
            board: None,
            best_move: None,
            channel_sender: tx,
            channel_receiver: rx,
            searcher: None,
        }
    }
}

impl Engine {
    pub fn start(self) -> (JoinHandle<()>, SyncSender<UciMessage>) {
        let tx1 = self.channel_sender.clone();

        (thread::spawn(|| self.run()), tx1)
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
            UciMessage::Stop => {
                match &self.board {
                    None => {},
                    Some(board) => {
                        let mut move_iter = MoveGen::new_legal(board);
                        match move_iter.next() {
                            Some(chess_move) => {
                                bestmove(chess_move, None)
                            },
                            None => {}
                        }
                    }
                }
            }

            UciMessage::PonderHit => {}
            UciMessage::Quit => {
                info!("Told to quit. Shutting down Threadpool...");
                // THREADS.quit();
                info!("Threadpool shut down.");
                return false;
            }
            UciMessage::Go {
                time_control,
                search_control,
            } => {
                rand_move(Option::as_ref(&self.board))
            }
            _ => {}
        }

        true
    }
}

fn calculate_time(time_control: UciTimeControl, to_move: Color) -> Option<Duration> {
    match time_control {
        UciTimeControl::MoveTime(duration) => duration.to_std().ok(),
        UciTimeControl::TimeLeft {
            white_time,
            black_time,
            moves_to_go,
            ..
        } => {
            match to_move {
                Color::White => white_time,
                Color::Black => black_time,
            }
            .map(|d| {
                //Convert from vampirc Duration to std duration.
                d.to_std().ok()
            })
            .flatten()
            .map(|d| {
                //Divide by moves until next time control or some sensible default
                d.div_f32(moves_to_go.unwrap_or(40) as f32)
            })
        }
        _ => None,
    }
}

fn rand_move(board: Option<&Board>) {
    match board {
        None => {},
        Some(some_board) => {
            let mut move_iter = MoveGen::new_legal(some_board);
            match move_iter.next() {
                Some(chess_move) => {
                    bestmove(chess_move, None)
                },
                None => {}
            }
        }
    }
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
