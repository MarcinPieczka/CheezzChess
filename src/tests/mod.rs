use crate::engine::calculate_time;
use chess::{Board, ChessMove, Color, Game, MoveGen};
use std::time::Duration;
use vampirc_uci::Duration as VampDuration;
use vampirc_uci::{UciInfoAttribute, UciMessage, UciTimeControl};

fn time_left(
    white_time: Option<i64>,
    white_increment: Option<i64>,
    black_time: Option<i64>,
    black_increment: Option<i64>,
) -> Option<UciTimeControl> {
    Some(UciTimeControl::TimeLeft {
        white_time: white_time.map(VampDuration::milliseconds),
        white_increment: white_increment.map(VampDuration::milliseconds),
        black_time: black_time.map(VampDuration::milliseconds),
        black_increment: black_increment.map(VampDuration::milliseconds),
        moves_to_go: None,
    })
}

#[test]
fn test_calculated_time_returns_move_time_when_present() {
    assert_eq!(
        calculate_time(time_left(Some(120), None, Some(120), None), Color::White),
        Duration::from_millis(3)
    )
}
