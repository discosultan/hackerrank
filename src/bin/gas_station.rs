use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn solution(a: &[i32], mut x: i32, mut y: i32, mut z: i32) -> i32 {
    let mut a_offset = 0;
    let mut time = -1;
    let mut x_occupied = 0;
    let mut y_occupied = 0;
    let mut z_occupied = 0;

    loop {
        // We iterate a unit of time.

        // 1. Fill tanks by a unit at occupied dispensers.
        if x_occupied > 0 {
            x_occupied -= 1;
            x -= 1;
        }
        if y_occupied > 0 {
            y_occupied -= 1;
            y -= 1;
        }
        if z_occupied > 0 {
            z_occupied -= 1;
            z -= 1;
        }

        // 2. Move all cars to available fuel dispensers.
        for &fuel_req in a.iter().skip(a_offset) {
            if x_occupied == 0 && x >= fuel_req {
                x_occupied = fuel_req;
                a_offset += 1;
            } else if y_occupied == 0 && y >= fuel_req {
                y_occupied = fuel_req;
                a_offset += 1;
            } else if z_occupied == 0 && z >= fuel_req {
                z_occupied = fuel_req;
                a_offset += 1;
            } else if x < fuel_req && y < fuel_req && z < fuel_req {
                // Special case to return -1 in case there's a car that's unable to fill it's tank.
                return -1;
            }
        }

        // 3. Advance time.
        // TODO: We can optimize here and advance time by min(x_occupied, y_occupied, z_occupied)
        // when they are all occupied. But we also need to adjust dispenser levels accordingly.
        time += 1;

        // 4. If none of the dispensers are occupied, it means there are no more cars in the queue.
        if x_occupied == 0 && y_occupied == 0 && z_occupied == 0 {
            break;
        }
    }

    time
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let a_count = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let mut a: Vec<i32> = Vec::with_capacity(a_count as usize);

    for _ in 0..a_count {
        let a_item = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();
        a.push(a_item);
    }

    let x = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let y = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let z = stdin_iterator.next().unwrap().unwrap().trim().parse::<i32>().unwrap();

    let result = solution(&a, x, y, z);

    writeln!(&mut fptr, "{}", result).ok();
}
