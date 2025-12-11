use std::io;
use std::io::Read;
use std::collections::HashMap;

use winnow::{
    Parser,
    Result,
    ascii::{ alpha1, newline },
    combinator::{
        repeat,
        separated,
        separated_pair,
        terminated
    }
};

#[derive(Default)]
struct Symbols {
    map: HashMap<String, usize>
}

impl Symbols {
    pub fn intern(&mut self, name: String) -> usize {
        match self.map.get(&name) {
            Some(&i) => i,
            None => {
                let i = self.map.len();
                self.map.insert(name, i);
                i
            }
        }
    }
}

struct Search {
    connections: Vec<Vec<usize>>,
    dac: usize,
    fft: usize,
    mem: HashMap<(usize, usize, bool, bool), usize>
}

impl Search {
    pub fn paths1(&self, start: usize, end: usize) -> usize {
        if start == end { return 1 }

        self.connections[start].iter()
            .map(|&output| self.paths1(output, end))
            .sum()
    }

    pub fn paths2(&mut self, start: usize, end: usize, dac_found: bool, fft_found: bool) -> usize {
        if start == end {
            return (dac_found && fft_found) as usize
        }

        match self.mem.get(&(start, end, dac_found, fft_found)) {
            Some(&result) => result,
            None => {
                let dac_found = dac_found || start == self.dac;
                let fft_found = fft_found || start == self.fft;

                let result = self.connections[start].clone().into_iter()
                    .map(|output| self.paths2(output, end, dac_found, fft_found))
                    .sum();

                self.mem.insert((start, end, dac_found, fft_found), result);
                result
            }
        }
    }
}

fn parse_device(input: &mut &str) -> Result<String> {
    alpha1.map(str::to_string).parse_next(input)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read error");

    let connections: Vec<(String, Vec<String>)> =
        repeat(1.., terminated(separated_pair(parse_device, ": ", separated(1.., parse_device, ' ')), newline))
        .parse(&input)
        .expect("parse error");

    let mut symbols: Symbols = Default::default();

    // Ensure devices with outputs are interned first
    // This allows us to discard the device id and
    // access them by index after sorting
    connections.iter().for_each(|(device, _)| { symbols.intern(device.to_owned()); });

    let mut connections: Vec<(usize, Vec<usize>)> = connections.into_iter().map(|(device, outputs)| {
        (
            symbols.intern(device),
            outputs.into_iter().map(|output| symbols.intern(output)).collect()
        )
    }).collect();

    connections.sort_unstable_by(|(device1, _), (device2, _)| device1.cmp(device2));

    let connections: Vec<Vec<usize>> = connections.into_iter().map(|(_device, outputs)| outputs).collect();

    let mut search = Search {
        connections,
        dac: symbols.intern(String::from("dac")),
        fft: symbols.intern(String::from("fft")),
        mem: HashMap::default()
    };

    let you = symbols.intern(String::from("you"));
    let svr = symbols.intern(String::from("svr"));
    let out = symbols.intern(String::from("out"));

    let result1 = search.paths1(you, out);
    println!("Part 1: {result1}");

    let result2 = search.paths2(svr, out, false, false);
    println!("Part 2: {result2}");
}
