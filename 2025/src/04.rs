use std::io;
use std::iter;

use ndarray::{ Array2 };

fn main() {
    let input: Vec<String> = io::stdin().lines()
        .map(|line| line.expect("read error"))
        .collect();

    let cols = input[0].len() + 2;
    let rows = input.len() + 2;

    let padded_cols = input.into_iter()
        .flat_map(|line| {
            iter::once(0u8)
                .chain(line.chars().map(|ch| (ch == '@') as u8))
                .chain(iter::once(0u8))
                .collect::<Vec<_>>()
        });

    let padded_rows = iter::repeat_n(0u8, cols)
        .chain(padded_cols)
        .chain(iter::repeat_n(0u8, cols));

    let mut current_map = Array2::from_shape_vec((rows, cols), padded_rows.collect::<Vec<_>>())
        .expect("wrong number of elements");

    let mut next_map = current_map.clone();

    let mut result1 = 0;
    let mut result2 = 0;

    loop {
        let mut removed = 0;

        for r in 1..(rows - 1) {
            for c in 1..(cols - 1) {
                if current_map[(r, c)] == 0 { continue }

                let mut adjacent = 0;
                for rr in (r - 1 )..=(r + 1) {
                    for cc in (c - 1)..=(c + 1) {
                        adjacent += current_map[(rr, cc)];
                    }
                }

                // 4 + center
                if adjacent < 5 {
                    next_map[(r, c)] = 0;
                    removed += 1;
                }
            }
        }
        if result1 == 0 { result1 = removed }
        result2 += removed;

        current_map.assign(&next_map);
        if removed == 0 { break }
    }

    println!("Part 1: {result1}");
    println!("Part 2: {result2}");
}
