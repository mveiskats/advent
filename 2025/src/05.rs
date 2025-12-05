use std::io;
use std::io::Read;
use std::str::FromStr;

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

fn parse_range(input: &mut &str) -> Result<(usize, usize)> {
    separated_pair(parse_number, '-', parse_number)
        .parse_next(input)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read error");

    let (mut ranges, ingredients): (Vec<(usize, usize)>, Vec<usize>) =
        (
            separated_pair(
                repeat(1.., terminated(parse_range, newline)),
                newline,
                repeat(1.., terminated(parse_number, newline))
            )
        ).parse(&input)
        .expect("parse error");

    let result1 = ingredients.into_iter()
        .filter(|i| {
            ranges.iter().any(|(start, end)| (start..=end).contains(&i))
        }).count();

    println!("Part 1: {result1}");

    ranges.sort_by(|(s1, _e1), (s2, _e2)| s1.cmp(s2));

    let mut covered = 0;
    let mut result2 = 0;

    for (start, end) in ranges {
        if end <= covered { continue }

        result2 += end - covered.max(start - 1);
        covered = end;
    }

    println!("Part 2: {result2}");
}
