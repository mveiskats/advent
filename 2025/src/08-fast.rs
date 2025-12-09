use std::io;
use std::io::Read;
use std::str::FromStr;

use glam::i64::I64Vec3 as Vec3;

use winnow::{
    Parser,
    Result,
    ascii::{ digit1, newline },
    combinator::{
        repeat,
        separated,
        terminated
    }
};

fn parse_number(input: &mut &str) -> Result<usize> {
    digit1.try_map(usize::from_str).parse_next(input)
}

fn parse_junction(input: &mut &str) -> Result<Vec<usize>> {
    separated(3..=3, parse_number, ',').parse_next(input)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read error");

    let junctions: Vec<Vec<usize>> =
        repeat(1.., terminated(parse_junction, newline))
        .parse(&input)
        .expect("parse error");

    let junctions: Vec<Vec3> = junctions.into_iter()
        .map(|v| Vec3::new(v[0] as i64, v[1] as i64, v[2] as i64))
        .collect();

    let count = junctions.len();

    let mut edges: Vec<(usize, usize, i64)> = Vec::with_capacity(count * (count + 1) / 2);

    for i in 0..count {
        for j in (i + 1)..count {
            let distance_squared: i64 = (junctions[i] - junctions[j]).length_squared();
            edges.push((i, j, distance_squared));
        }
    }

    let mut junction_circuits: Vec<usize> = (0..count).collect();
    let mut circuit_sizes: Vec<usize> = vec![1; count];

    let edge_count = edges.len();
    let sort_chunk = 100000;

    let mut connection = 0;
    let mut unsorted = 0;

    loop {
        if connection == 1000 {
            let mut result1 = circuit_sizes.clone();
            result1.sort();

            let result1: usize = result1.into_iter().rev().take(3).product();
            println!("Part 1: {result1}");
        }

        // Betting we won't need to process most of the edges by partial sorting.
        // If we have to go through most of them, we'll end up being slower
        // than sorting everything at once
        if unsorted <= connection {
            let remaining = edge_count - unsorted;
            if remaining < sort_chunk {
                edges[unsorted..].sort_unstable_by(|(_i1, _j1, d1), (_i2, _j2, d2)| d1.cmp(d2));
                unsorted = edge_count;
            } else {
                let (lesser, _pivot, _greater) = edges[unsorted..]
                    .select_nth_unstable_by(sort_chunk, |(_i1, _j1, d1), (_i2, _j2, d2)| d1.cmp(d2));

                lesser.sort_unstable_by(|(_i1, _j1, d1), (_i2, _j2, d2)| d1.cmp(d2));
                unsorted += sort_chunk;
            }
        }

        let (i, j, _distance) = edges[connection];

        let into = junction_circuits[i];
        let from = junction_circuits[j];

        if into != from {
            junction_circuits.iter_mut()
                .filter(|a| **a == from)
                .for_each(|a| {
                    *a = into;
                    circuit_sizes[into] += 1;
                    circuit_sizes[from] -= 1;
                });

            if circuit_sizes.iter().filter(|&&size| size > 0).count() == 1 {
                let result2 = junctions[i][0] * junctions[j][0];
                println!("Part 2: {result2}");

                break
            }
        }

        connection += 1;
    }
}
