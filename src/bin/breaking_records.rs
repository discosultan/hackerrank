use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn breaking_records(scores: &[i32]) -> Vec<i32> {
    if scores.len() <= 1 {
        return vec![0, 0];
    }

    let mut min = scores[0];
    let mut max = min;

    let mut min_breaks = 0;
    let mut max_breaks = 0;

    for &score in scores {
        if score < min {
            min = score;
            min_breaks += 1;
        } else if score > max {
            max = score;
            max_breaks += 1;
        }
    }

    vec![max_breaks, min_breaks]
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let _n = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .parse::<i32>()
        .unwrap();

    let scores: Vec<i32> = stdin_iterator
        .next()
        .unwrap()
        .unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let result = breaking_records(&scores);

    for i in 0..result.len() {
        write!(&mut fptr, "{}", result[i]).ok();

        if i != result.len() - 1 {
            write!(&mut fptr, " ").ok();
        }
    }

    writeln!(&mut fptr).ok();
}
