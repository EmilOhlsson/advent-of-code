use std::collections::{HashMap, HashSet};

fn solve(input: &str) -> (u32, u32) {
    let mut orbits = HashMap::new();
    let mut planets = HashSet::new();
    for line in input.lines() {
        let toks = line.split(')').collect::<Vec<_>>();
        orbits.insert(toks[1].to_string(), toks[0].to_string());
        planets.insert(toks[1].to_string());
    }

    // Part 1
    let mut tot = 0;
    for planet in &planets {
        let mut tmp = planet;
        while let Some(p) = orbits.get(tmp) {
            tmp = p;
            tot += 1;
        }
    }

    // Part 2
    let mut san = HashSet::new();
    let mut tmp = "SAN";
    while let Some(p) = orbits.get(tmp) {
        san.insert(p);
        tmp = p;
    }
    let mut you = HashSet::new();
    let mut tmp = "YOU";
    while let Some(p) = orbits.get(tmp) {
        you.insert(p);
        tmp = p;
    }

    (tot, you.symmetric_difference(&san).count() as u32)
}

fn main() {
    let input = include_str!("input");
    println!("{:?}", solve(input));
}

#[test]
fn test() {
    let input = include_str!("input-test");
    assert_eq!(solve(input).0, 42);
}

#[test]
fn test_p2() {
    let input = include_str!("input-test-p2");
    assert_eq!(solve(input).1, 4);
}
