use std::io;
use std::io::prelude::*;

fn main() {
    let input = io::stdin();
    // let mut input_handle = input.lock();
    let output = io::stdout();
    let mut output_handle = output.lock();

    let mut buffer = String::new();
    input.read_line(&mut buffer);

    let x = buffer
        .split_whitespace()
        .map(|w| { w.parse::<u32>().unwrap() })
        .collect::<Vec<_>>();

    let mut x1 = x[0];
    let v1 = x[1];
    let mut x2 = x[2];
    let v2 = x[3];

    if x1 == x2 {
        output_handle.write(b"YES");
        return;
    }

    if v1 <= v2 {
        output_handle.write(b"NO");
        return;
    }

    while x1 < x2 {
        x1 += v1;
        x2 += v2;

        if x1 == x2 {
            output_handle.write(b"YES");
            return;
        }
    }

    output_handle.write(b"NO");
}
