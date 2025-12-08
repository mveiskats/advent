use std::io;
use std::io::Read;
use std::str::FromStr;

use ndarray::Array2;

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

    let count = junctions.len();

    let mut distances = Array2::from_elem((junctions.len(), junctions.len()), usize::MAX);

    for i in 0..count {
        for j in (i + 1)..count {
            let distance_squared: usize = junctions[i].iter().zip(junctions[j].iter())
                .map(|(a, b)| a.abs_diff(*b))
                .map(|delta| delta * delta)
                .sum();

            distances[(i, j)] = distance_squared;
        }
    }

    let mut edges: Vec<((usize, usize), usize)> = distances.indexed_iter()
        .map(|((i, j), distance)| ((i, j), *distance))
        .filter(|((i, j), _distance)| j > i)
        .collect();

    edges.sort_by(|((_i1, _j1), d1), ((_i2, _j2), d2)| d1.cmp(d2));

    let mut junction_circuits: Vec<usize> = (0..count).collect();
    let mut circuit_sizes: Vec<usize> = vec![1; count];

    for (connection, ((i, j), _distance)) in edges.into_iter().enumerate() {
        if connection == 1000 {
            let mut result1 = circuit_sizes.clone();
            result1.sort();

            let result1: usize = result1.into_iter().rev().take(3).product();
            println!("Part 1: {result1}");
        }

        let into = junction_circuits[i];
        let from = junction_circuits[j];

        if into == from { continue }

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
}
