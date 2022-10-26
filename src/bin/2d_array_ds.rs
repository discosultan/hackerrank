use std::io;
use std::io::prelude::*;

const LEN: usize = 6;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines().flat_map(|l| {
        l.unwrap()
            .split_whitespace()
            .map(|w| w.parse::<i32>().unwrap())
            .collect::<Vec<_>>()
            .into_iter()
    });

    let mut a = [[0; LEN]; LEN];

    // for y in 0..LEN {
    //     for x in 0..LEN {
    //         a[x][y] = lines.next().unwrap();
    //     }
    // }
    for x_val in a.iter_mut().take(LEN) {
        for y_val in x_val.iter_mut().take(LEN) {
            *y_val = lines.next().unwrap();
        }
    }

    let ans = algorithm(a);
    println!("{}", ans);
}

fn algorithm(a: [[i32; LEN]; LEN]) -> i32 {
    let mut max_sum = std::i32::MIN;

    for y in 0..LEN - 2 {
        for x in 0..LEN - 2 {
            let mut sum = 0;
            sum += a[x][y];
            sum += a[x + 1][y];
            sum += a[x + 2][y];
            sum += a[x + 1][y + 1];
            sum += a[x][y + 2];
            sum += a[x + 1][y + 2];
            sum += a[x + 2][y + 2];
            if sum > max_sum {
                max_sum = sum;
            }
        }
    }

    max_sum
}
