use fibers::io::stdin;
use log::info;
use simplelog::{Config, LevelFilter, WriteLogger};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use vampirc_uci::parse_one;

use engine::Engine;

mod engine;

fn main() {
    let path = Path::new("engine.log");
    let logfile = File::create(path).ok().unwrap();
    let _ = WriteLogger::init(LevelFilter::Info, Config::default(), logfile);

    let mut input = String::new();
    let running = Arc::new(AtomicBool::new(true));
    let (handle, tx) = {
        let engine = Engine::default();
        engine.start()
    };
    let mut stdin = BufReader::new(stdin());

    while running.load(Ordering::Acquire) {
        if stdin.read_line(&mut input).is_err() {
            thread::sleep(Duration::from_millis(2));
        } else {
            if input.starts_with("quit") {
                running.store(false, Ordering::Release);
            }
            let message = parse_one(&input);

            if tx.send(message).is_err() {
                break;
            }

            input.clear();
        }
    }

    info!("Joining engine controller thread...");
    handle.join().ok();
}
