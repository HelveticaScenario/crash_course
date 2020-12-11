use std::{env::args, process};

pub fn main() {
    let args: Vec<_> = args().collect();
    let (sum, v) = args
        .iter()
        .skip(1)
        .take(2)
        .map(|arg| {
            arg.parse::<i32>().unwrap_or_else(|_| {
                println!("Usage");
                process::exit(1);
            })
        })
        .fold((0, vec![]), |(acc, mut v), i| {
            v.push(i);
            (acc + i, v)
        });

    print!(
        "{} = {}",
        v.iter()
            .map(|i| i.to_string())
            .collect::<Vec<String>>()
            .join(" + "),
        sum
    );
}
