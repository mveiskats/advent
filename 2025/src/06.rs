use std::io;
use ndarray::{ Array2, ArrayView, Axis, Ix2, s };

fn maths(op: char, chunk: ArrayView<char, Ix2>) -> usize {
    let numbers = chunk.axis_iter(Axis(0)).map(|row| {
        row.iter()
            .filter(|&&ch| ch != ' ')
            .collect::<String>()
            .parse::<usize>()
            .expect("not a number")
    });

    match op {
        '+' => numbers.sum(),
        '*' => numbers.product(),
        _ => panic!("unknown operator")
    }
}

fn main() {
    let lines: Vec<String> = io::stdin().lines()
        .map(|line| line.expect("read error"))
        .collect();

    let operators: Vec<char> = lines[lines.len() - 1]
        .chars()
        .filter(|&ch| ch != ' ')
        .collect();

    let rows = lines.len() - 1;
    let cols = lines[0].len();

    let digit_grid = Array2::from_shape_vec(
        (rows, cols),
        lines[..(lines.len() - 1)].iter().flat_map(|s| s.chars()).collect()
    ).expect("wrong vector size");

    let mut result1 = 0;
    let mut result2 = 0;

    let mut chunk_start = 0;

    for op in operators {
        let mut chunk_end = chunk_start;
        while chunk_end < cols && digit_grid.slice(s![.., chunk_end]).iter().any(|&ch| ch != ' ') {
            chunk_end += 1;
        }

        let chunk = digit_grid.slice(s![.., chunk_start..chunk_end]);

        result1 += maths(op, chunk);
        result2 += maths(op, chunk.t());

        chunk_start = chunk_end + 1;
    }

    println!("Part 1: {result1}");
    println!("Part 2: {result2}");
}
