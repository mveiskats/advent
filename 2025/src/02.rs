use std::io;
use std::io::Read;
use std::str::FromStr;

use winnow::{
    Parser,
    Result,
    ascii::digit1,
    combinator::{ separated, separated_pair }
};


fn parse_range(input: &mut &str) -> Result<(usize, usize)> {
    separated_pair(
        digit1.try_map(usize::from_str),
        '-',
        digit1.try_map(usize::from_str)
    ).parse_next(input)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read error");

    let ranges: Vec<(usize, usize)> = separated(1.., parse_range, ',')
        .parse(&input)
        .expect("parse error");

    let mut result1: usize = 0;
    let mut result2: usize = 0;

    for (from, to) in ranges.into_iter() {
        'id: for id in from..=to {
            let mut divider = 1;

            'split: loop {
                divider *= 10;
                let first = id % divider;
                let mut rem = id / divider;

                if rem == 0 { continue 'id }

                // skip parts starting with zero
                if first < divider / 10 { continue 'split }

                let mut count = 1;

                while rem > 0 {
                    // one of the chunks doesn't match
                    if first != rem % divider { continue 'split }

                    count +=1;
                    rem /= divider;
                }

                if count == 2 { result1 += id; }
                result2 += id;

                continue 'id
            }
        }
   }

    println!("Part 1: {result1}");
    println!("Part 2: {result2}");
}
