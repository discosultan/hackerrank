use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

const ACCESSORS: &[&[usize]] = &[
    // // Horizontals.
    // &[(0, 0), (0, 1), (0, 2)],
    // &[(1, 0), (1, 1), (1, 2)],
    // &[(2, 0), (2, 1), (2, 2)],
    // // Verticals.
    // &[(0, 0), (1, 0), (2, 0)],
    // &[(0, 1), (1, 1), (2, 1)],
    // &[(0, 2), (1, 2), (2, 2)],
    // // Diagonals.
    // &[(0, 0), (1, 1), (2, 2)],
    // &[(2, 0), (1, 1), (0, 2)],

     // Horizontals.
     &[0, 1, 2],
     &[3, 4, 5],
     &[6, 7, 8],
     // Verticals.
     &[0, 3, 6],
     &[1, 4, 7],
     &[2, 5, 8],
     // Diagonals.
     &[0, 4, 8],
     &[6, 4, 2],
];

fn is_magic(s: &[u32]) -> bool {
    let mut magic_value = None;
    for accessor in ACCESSORS {
        let mut line_val = 0;
        for &i in accessor.iter() {
            line_val += s[i];
        }
        match magic_value {
            None => {
                magic_value = Some(line_val);
            },
            Some(magic_value) => if magic_value != line_val { return false; },
        }
    }
    true
}

fn calc_cost(s1: &[u32], s2: &[u32]) -> u32 {
    let mut cost = 0;
    for i in 0..9 {
        // The following was not available on hackerrank due to older rust version.
        // cost += s1[i].abs_diff(s2[i]);

        cost += (s1[i] as i32 - s2[i] as i32).unsigned_abs();
    }
    cost
}

// https://www.geeksforgeeks.org/heaps-algorithm-for-generating-permutations/
fn heap_permutation(s: &[u32], best_cost: &mut u32, a: &mut [u32], size: usize) {
    // if size becomes 1 then prints the obtained permutation
    if size == 1 {
        if is_magic(a) {
            *best_cost = (*best_cost).min(calc_cost(s, a));
        }
        return;
    }

    for i in 0..size {
        heap_permutation(s, best_cost, a, size - 1);

        // if size is odd, swap 0th i.e (first)
        // and (size-1)th i.e (last) element
        // else If size is even, swap ith
        // and (size-1)th i.e (last) element
        if size % 2 == 1 {
            (a[0], a[size - 1]) = (a[size - 1], a[0]);
        } else {
            (a[i], a[size - 1]) = (a[size - 1], a[i]);
        }
    }
}

fn forming_magic_square(s: &[Vec<u32>]) -> u32 {
    // Simplify to a 1D array.
    let s = s.iter().flatten().cloned().collect::<Vec<_>>();

    let mut best_cost = u32::MAX;
    heap_permutation(&s, &mut best_cost, &mut [1, 2, 3, 4, 5, 6, 7, 8, 9], 9);
    best_cost
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let mut s: Vec<Vec<u32>> = Vec::with_capacity(3_usize);

    for i in 0..3_usize {
        s.push(Vec::with_capacity(3_usize));

        s[i] = stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.to_string().parse::<u32>().unwrap())
            .collect();
    }

    let result = forming_magic_square(&s);

    writeln!(&mut fptr, "{}", result).ok();
}
