extern crate regex;

use regex::Regex;
use std::collections::HashMap;

struct Claim {
    id: usize,
    pos: (usize, usize),
    size: (usize, usize),
}

fn solve(input: &str) -> usize {
    let re = Regex::new(r"(\d+)").unwrap();
    let claims = input
        .lines()
        .map(|l| {
            let ts = re
                .captures_iter(l)
                .map(|c| c[1].parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            Claim {
                id: ts[0],
                pos: (ts[1], ts[2]),
                size: (ts[3], ts[4]),
            }
        }).collect::<Vec<Claim>>();

    let mut fabric: HashMap<(usize, usize), usize> = HashMap::new();
    for claim in &claims {
        for y in claim.pos.1..(claim.pos.1 + claim.size.1) {
            for x in claim.pos.0..(claim.pos.0 + claim.size.0) {
                let cls = fabric.entry((x, y)).or_insert(0);
                *cls += 1;
            }
        }
    }

    fabric.values().filter(|&v| *v > 1).count()
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", solve(input));
}
