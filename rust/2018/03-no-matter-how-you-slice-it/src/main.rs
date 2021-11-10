extern crate regex;

use regex::Regex;
use std::collections::{HashMap, HashSet};

struct Claim {
    pos: (usize, usize),
    size: (usize, usize),
}

fn solve(input: &str) -> (usize, Vec<usize>) {
    let re = Regex::new(r"(\d+)").unwrap();
    let claims = input
        .lines()
        .map(|l| {
            let ts = re
                .captures_iter(l)
                .map(|c| c[1].parse::<usize>().unwrap())
                .collect::<Vec<usize>>();
            (
                ts[0],
                Claim {
                    pos: (ts[1], ts[2]),
                    size: (ts[3], ts[4]),
                },
            )
        }).collect::<HashMap<usize, Claim>>();

    let mut fabric: HashMap<(usize, usize), Vec<usize>> = HashMap::new();

    let mut overlaps: HashSet<usize> = HashSet::new();
    let ids: HashSet<usize> = claims.keys().map(|i| *i).collect();
    for (id, claim) in &claims {
        for y in claim.pos.1..(claim.pos.1 + claim.size.1) {
            for x in claim.pos.0..(claim.pos.0 + claim.size.0) {
                let ids = fabric.entry((x, y)).or_insert(Vec::new());
                ids.push(*id);
                if ids.len() > 1 {
                    for id in ids {
                        overlaps.insert(*id);
                    }
                }
            }
        }
    }

    (
        fabric.values().filter(|&v| v.len() > 1).count(),
        ids.difference(&overlaps).map(|i| *i).collect(),
    )
}

fn main() {
    let input = include_str!("input.txt");
    let (p1, p2) = solve(input);
    println!("{} - {:?}", p1, p2);
}
