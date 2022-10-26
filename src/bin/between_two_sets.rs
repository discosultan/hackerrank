use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    let stdin = stdin.lock();
    let stdout = io::stdout();
    let stdout = stdout.lock();

    let mut values = stdin
        .lines()
        .map(|l| { l.unwrap() })
        .flat_map(|l| {
            l.split_whitespace()
             .map(|w| w.parse::<usize>().unwrap())
             .collect::<Vec<_>>()
             .into_iter()
        });

    let n = values.next().unwrap();
    let m = values.next().unwrap();

    let mut A = vec![0; n];
    for i in 0..n {
        A[i] = values.next().unwrap();
    }

    let mut B = vec![0; m];
    for i in 0..m {
        B[i] = values.next().unwrap();
    }

    let lcm = lcm_set(&A);
    let gcd = gcd_set(&B);
    // println!("{}", lcm);
    // println!("{}", gcd);

    let ans = count_between(lcm, gcd);
    println!("{}", ans);
}

fn gcd_set(set: &Vec<usize>) -> usize {
    set.iter().fold(0, |res, &val| gcd(res, val))
}

fn gcd(mut x: usize, mut y: usize) -> usize {
    while y > 0 {
        // RFC for destructuring assignment: https://github.com/rust-lang/rfcs/issues/372
        // (a, b) = (b, a % b);

        let temp = x;
        x = y;
        y = temp % y;
    }
    x
}

fn lcm_set(set: &Vec<usize>) -> usize {
    set.iter().fold(1, |res, &val| lcm(res, val))
}

fn lcm(x: usize, y: usize) -> usize {
    (x * y) / gcd(x, y)
}

// assume x <= y
fn count_between(mut x: usize, y: usize) -> usize {
    let mut cnt = 0;
    let mut walk = x;
    while walk <= y {
        if y % walk == 0 {
            cnt += 1;
        }
        walk += x;
    }
    cnt
}
