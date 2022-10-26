use std::io;
use std::io::prelude::*;

fn main() {
    let input = io::stdin();
    let mut input_handle = input.lock();
    let output = io::stdout();
    let mut output_handle = output.lock();

    let mut values = input_handle.lines().map(|value| {
        value.unwrap().trim().parse::<u32>().unwrap()
    });
    let count = values.next().unwrap();
    // println!("{:?}", count);

    let mut values = values
        .take(count as usize)
        .map(|value| {
            let mut result = value;
            if value > 37 {
                let diff = 5 - value % 5;
                if diff <= 2 {
                    result += diff;
                }
            }
            result
        });

    for value in values {
        output_handle.write(value.to_string().as_bytes());
        output_handle.write(b"\n");
    }
}
