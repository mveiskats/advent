use std::io;

fn main() {
    let lines: Vec<String> = io::stdin().lines()
        .map(|line| line.expect("read error"))
        .collect();

    let cols = lines[0].len();

    let mut beams: Vec<usize> = vec![0; cols];
    let mut new_beams: Vec<usize>;

    let mut result1 = 0;

    for line in lines {
        new_beams = vec![0; cols];

        for (col, ch) in line.chars().enumerate() {
            match ch {
                'S' => {
                    new_beams[col] += 1;
                },
                '^' => {
                    let current = beams[col];
                    if current == 0 { continue  }

                    new_beams[col - 1] += current;
                    new_beams[col + 1] += current;

                    result1 += 1;
                },
                _ => {
                    new_beams[col] += beams[col];
                }
            }
        }

        beams = new_beams;
    }

    let result2: usize = beams.iter().sum();

    println!("Part 1: {result1}");
    println!("Part 2: {result2}");
}
