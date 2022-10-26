use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let stdout = io::stdout();
    let stdout = stdout.lock();

    let mut lines = stdin
        .lines()
        .flat_map(|l| {
            l
                .unwrap()
                .split_whitespace()
                .map(|w| { w.parse::<usize>().unwrap() })
                .collect::<Vec<_>>()
                .into_iter()
        });

    let N = lines.next().unwrap();
    let A = lines.collect::<Vec<_>>();

    for val in A.iter().rev() {
        print!("{}", val);
        print!(" ");
    }
}
