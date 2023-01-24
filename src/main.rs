use fibers::io::stdin;
use simplelog::{Config, LevelFilter, WriteLogger};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::env;
use vampirc_uci::parse_one;

use engine::Engine;

mod engine;

#[cfg(not(test))] 
use log::{info, warn};
 
#[cfg(test)]
use std::{println as info, println as warn};

fn main() {
    let path = Path::new("engine.log");
    let logfile = File::create(path).ok().unwrap();
    let _ = WriteLogger::init(LevelFilter::Info, Config::default(), logfile);

    let mut input = String::new();
    let running = Arc::new(AtomicBool::new(true));
    let mut depth = 3;
    let args: Vec<String> = env::args().collect();
    if args.len() >= 2 {
        match args[1].parse::<u8>() {
            Ok(num) => {
                depth = num;
            },
            _ => {panic!("first argument if provided should be depth")}
        }
    }

    let (handle, tx) = {
        let engine = Engine::default();
        engine.start(depth)
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
