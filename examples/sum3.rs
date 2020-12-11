use std::{io, thread, time::Duration};
use std::io::prelude::*;
use std::io::BufReader;
use std::{env::args, fs::File};
pub fn main() -> io::Result<()> {
    let f = File::open(args().collect::<Vec<_>>().get(1).unwrap())?;
    let reader = BufReader::new(f);
    let sum = reader
        .lines()
        .map(|l| l.unwrap().parse::<u128>().unwrap())
        .fold(1u128, |acc, i| acc + i);
    println!("{}", sum);
    Ok(())
}
