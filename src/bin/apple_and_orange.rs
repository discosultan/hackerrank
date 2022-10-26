use std::io;
use std::io::prelude::*;

fn main() {
    let input = io::stdin();
    let mut input_handle = input.lock();
    let output = io::stdout();
    let mut output_handle = output.lock();

    let mut values = input_handle
        .lines()
        .map(|l| { l.unwrap() })
        .flat_map(|l| {
            l.split_whitespace()
             .map(|w| w.parse::<i32>().unwrap())
             .collect::<Vec<_>>()
             .into_iter()
        });

    let s = values.next().unwrap();
    let t = values.next().unwrap();
    let a = values.next().unwrap();
    let b = values.next().unwrap();
    let m = values.next().unwrap();
    let n = values.next().unwrap();

    let apples_on_house = count_on_house(a, s, t, m, &mut values);
    let oranges_on_house = count_on_house(b, s, t, n, &mut values);

    output_handle.write(apples_on_house.to_string().as_bytes());
    output_handle.write(b"\n");
    output_handle.write(oranges_on_house.to_string().as_bytes());
    output_handle.write(b"\n");
}

fn count_on_house<I>(origin: i32, s: i32, t: i32, count: i32, values: &mut I) -> i32 
    where I: Iterator<Item=i32>
{
    let mut result = 0;
    for _ in 0..count {
        let d = values.next().unwrap();
        let p = origin + d;
        if p >= s && p <= t {
            result += 1;
        }
    }
    result
}
