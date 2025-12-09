use std::io;
use std::io::Read;
use std::fmt;
use std::collections::VecDeque;
use std::str::FromStr;

use ndarray::{ Array2, s };

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Other,
    Red,
    Green
}

impl fmt::Debug for Tile {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ch = match self {
            Tile::Other => '.',
            Tile::Red => '#',
            Tile::Green => 'X'
        };
        write!(f, "{ch}")
    }
}

use winnow::{
    Parser,
    Result,
    ascii::{ digit1, newline },
    combinator::{
        repeat,
        separated_pair,
        terminated
    }
};

fn parse_number(input: &mut &str) -> Result<usize> {
    digit1.try_map(usize::from_str).parse_next(input)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read error");

    let corners: Vec<(usize, usize)> =
        repeat(1.., terminated(separated_pair(parse_number, ',', parse_number), newline))
        .parse(&input)
        .expect("parse error");

    let mut compressed_corners = corners.clone();

    // Compress x axis
    let mut start = 0;

    loop {
        let end = compressed_corners.iter().copied()
            .map(|(x, _y)| x)
            .filter(|x| *x > start)
            .min();

        if let Some(end) = end {
            if end > start {
                compressed_corners.iter_mut().for_each(|(x, _y)| {
                    if *x > start { *x -= end - start }
                });
            }

            start += 1;
        } else {
            break
        }
    }

    // Compress y axis
    let mut start = 0;

    loop {
        let end = compressed_corners.iter().copied()
            .map(|(_x, y)| y)
            .filter(|y| *y > start)
            .min();

        if let Some(end) = end {
            if end > start {
                compressed_corners.iter_mut().for_each(|(_x, y)| {
                    if *y > start { *y -= end - start }
                });
            }

            start += 1;
        } else {
            break
        }
    }

    // Expand x axis a little
    let times = 0;

    for i in 0..compressed_corners.iter().copied().map(|(x, _y)| x).max().expect("no input tiles") {
        let start = i * 2 + 1;
        compressed_corners.iter_mut().for_each(|(x, _y)| {
            if *x >= start { *x += 1 }
        });
    }

    // Expand y axis a little
    let times = 0;

    for i in 0..compressed_corners.iter().copied().map(|(_x, y)| y).max().expect("no input tiles") {
        let start = i * 2 + 1;
        compressed_corners.iter_mut().for_each(|(_x, y)| {
            if *y >= start { *y += 1 }
        });
    }

    // Fill the edges
    let cols = compressed_corners.iter().map(|(x, _y)| x).max().expect("no input tiles") + 1;
    let rows = compressed_corners.iter().map(|(_x, y)| y).max().expect("no input tiles") + 1;
    let mut tiles = Array2::from_elem((rows, cols), Tile::Other);

    for i in 0..compressed_corners.len() {
        let (x1, y1) = compressed_corners[i];
        tiles[(y1, x1)] = Tile::Red;

        let j = if i == compressed_corners.len() - 1 { 0 } else { i + 1 };
        let (x2, y2) = compressed_corners[j];

        tiles.slice_mut(s![y1.min(y2)..=y1.max(y2), x1.min(x2)..=x1.max(x2)]).iter_mut().for_each(|t| {
            if *t == Tile::Other { *t = Tile::Green }
        });
    }

    // Fill interior
    let top_left_corner = tiles.slice(s![0, ..]).iter().copied()
        .position(|t| t == Tile::Red )
        .expect("no red tiles on first row");

    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    queue.push_back((1, top_left_corner + 1));

    while let Some((row, col)) = queue.pop_front() {
        if tiles[(row, col)] != Tile::Other { continue }
        tiles[(row, col)] = Tile::Green;
        queue.push_back((row - 1, col));
        queue.push_back((row + 1, col));
        queue.push_back((row, col + 1));
        queue.push_back((row, col - 1));
    }

    let mut result2 = 0;
    let mut result1 = 0;

    for i in 0..corners.len() {
        for j in (i + 1)..corners.len() {
            let area = (corners[i].0.abs_diff(corners[j].0) + 1) * (corners[i].1.abs_diff(corners[j].1) + 1);
            result1 = result1.max(area);

            let (x1, y1) = compressed_corners[i];
            let (x2, y2) = compressed_corners[j];

            let other_tiles = tiles
                .slice(s![y1.min(y2)..=y1.max(y2), x1.min(x2)..=x1.max(x2)])
                .iter()
                .filter(|t| **t == Tile::Other)
                .count();

            if other_tiles > 0 { continue }

            let area = (corners[i].0.abs_diff(corners[j].0) + 1) * (corners[i].1.abs_diff(corners[j].1) + 1);
            result2 = result2.max(area);
        }
    }

    println!("Part 1: {result1}");
    println!("Part 2: {result2}");
}
