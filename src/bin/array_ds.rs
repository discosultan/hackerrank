use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let stdout = io::stdout();
    let _stdout = stdout.lock();

    let mut lines = stdin.lines().flat_map(|l| {
        l.unwrap()
            .split_whitespace()
            .map(|w| w.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
            .into_iter()
    });

    let _n = lines.next().unwrap();
    let a = lines.collect::<Vec<_>>();

    for val in a.iter().rev() {
        print!("{}", val);
        print!(" ");
    }
}
