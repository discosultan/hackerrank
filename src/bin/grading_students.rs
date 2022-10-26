use std::io;
use std::io::prelude::*;

fn main() {
    let input = io::stdin();
    let input_handle = input.lock();
    let output = io::stdout();
    let mut output_handle = output.lock();

    let mut values = input_handle
        .lines()
        .map(|value| value.unwrap().trim().parse::<u32>().unwrap());
    let count = values.next().unwrap();

    let values = values.take(count as usize).map(|value| {
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
        output_handle
            .write_all(value.to_string().as_bytes())
            .unwrap();
        output_handle.write_all(b"\n").unwrap();
    }
}
