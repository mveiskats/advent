use std::io;

fn main() {
    let mut position: isize = 50;
    let mut result1: usize = 0;
    let mut result2: usize = 0;

    io::stdin().lines()
        .map(|line| line.expect("read error"))
        .for_each(|line| {
            let direction = line.chars().next().expect("empty line");
            let distance = line[1..].parse::<isize>().expect("not a number");

            let direction: isize = match direction {
                'R' => 1,
                'L' => -1,
                _ => panic!("unknown direction")
            };

            for _ in 0..distance {
                position = (position + direction).rem_euclid(100);
                if position == 0 { result2 += 1 }
            }

            if position == 0 { result1 += 1 }
        });

    println!("Result 1: {result1}");
    println!("Result 2: {result2}");
}
