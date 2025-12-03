use std::io;

fn max_joltage(bank: &[u8], cells: usize, joltage: usize) -> usize {
    if cells == 0 { return joltage }

    let range = ..=(bank.len() - cells);
    let max: u8 = *bank[range].iter().max().expect("empty bank");
    let max_pos = bank[range].iter()
        .position(|&x| x == max)
        .expect("where did it go?");

    max_joltage(&bank[(max_pos + 1)..], cells - 1, joltage * 10 + max as usize)
}

fn main() {
    let input: Vec<Vec<u8>> = io::stdin().lines()
        .map(|line| line.expect("read error"))
        .map(|line| {
            line.chars().map(|ch| {
                ch.to_digit(10).expect("not a digit") as u8
            }).collect()
        }).collect();

    let result1: usize = input.iter()
        .map(|bank| max_joltage(bank, 2, 0))
        .sum();

    let result2: usize = input.iter()
        .map(|bank| max_joltage(bank, 12, 0))
        .sum();

    println!("Part 1: {result1}");
    println!("Part 2: {result2}");
}
