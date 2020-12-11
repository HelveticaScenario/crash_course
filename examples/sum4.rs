use std::io::prelude::*;
use std::io::BufReader;
use std::sync::mpsc::{channel, Receiver};
use std::{env::args, fs::File};
use std::{
    io::{self, Stdout},
    thread,
    time::Duration,
};

use crossterm::{cursor, QueueableCommand, Result};
use io::stdout;

const FRAMES: [char; 10] = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'];

struct Spinner {
    stdout: Stdout,
    rx: Receiver<()>,
    frame: usize,
}

impl Spinner {
    fn new(rx: Receiver<()>) -> Self {
        Spinner {
            stdout: stdout(),
            rx,
            frame: 0,
        }
    }
    fn spin(mut self) -> Result<()> {
        loop {
            self.inc()?;
            match self.rx.recv_timeout(Duration::from_millis(500)) {
                Ok(_) => return Ok(()),
                Err(_) => {}
            }
        }
    }
    fn inc(&mut self) -> Result<()> {
        self.stdout.queue(cursor::SavePosition)?;
        self.stdout
            .write(format!("{}", FRAMES[self.frame % FRAMES.len()]).as_bytes())?;
        self.stdout.queue(cursor::RestorePosition)?;
        self.stdout.flush()?;
        self.frame = (self.frame + 1) % FRAMES.len();
        Ok(())
    }
}

pub fn main() -> io::Result<()> {
    let f = File::open(args().collect::<Vec<_>>().get(1).unwrap())?;
    let (tx, rx) = channel();
    let _handle = thread::spawn(move || {
        Spinner::new(rx).spin().unwrap();
    });
    let reader = BufReader::new(f);
    let sum = reader
        .lines()
        .map(|l| l.unwrap().parse::<u128>().unwrap())
        .fold(1u128, |acc, i| acc + i);
    tx.send(()).unwrap();
    _handle.join().unwrap();
    println!("{}", sum);
    Ok(())
}
