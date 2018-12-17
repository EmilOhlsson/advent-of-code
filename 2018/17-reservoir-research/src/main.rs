use std::collections::HashMap;

use regex::Regex;
use coding_challenge_utils::coord::Cartesian;

#[derive(Debug, Hash)]
enum Grid {
    Clay,
    Water,
}

fn solve(input: &str) -> usize {
    let mut grid: HashMap<Cartesian, Grid> = HashMap::new();
    let re = Regex::new(r"(?m)^([xy])=(\d+), ([xy])=(\d+)\.\.(\d+)$").unwrap();
    for cap in re.captures_iter(input) {
        match cap[1] {
            "x" => {}
            "y" => {}
            _ => panic!(),
        }
    }

    unimplemented!();
}

fn main() {
    let input = include_str!("input.txt");
    println!("{}", solve(input));
}

#[test]
fn test() {
    let input = include_str!("input-simple.txt");
    assert_eq!(solve(input), 57);
}
