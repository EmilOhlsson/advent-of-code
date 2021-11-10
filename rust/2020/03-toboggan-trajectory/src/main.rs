use std::collections::HashSet;

fn solve(input: &str) -> u32 {
    let mut cols = 0;
    let rows = input.lines().count();
    let mut map = HashSet::<(usize, usize)>::new();
    for (r, line) in input.lines().enumerate() {
        cols = line.len();
        for (c, ch) in line.chars().enumerate() {
            if ch == '#' {
                map.insert((r, c));
            }
        }
    }

    let mut trees = 0;
    let mut pos = (0, 0);

    while pos.0 < rows {
        pos = (pos.0 + 1, pos.1 + 3);
        if map.contains(&(pos.0, pos.1 % cols)) {
            trees += 1
        }
    }

    trees
}

fn solve_p2(input: &str) -> u64 {
    let mut cols = 0;
    let rows = input.lines().count();
    let mut map = HashSet::<(usize, usize)>::new();
    for (r, line) in input.lines().enumerate() {
        cols = line.len();
        for (c, ch) in line.chars().enumerate() {
            if ch == '#' {
                map.insert((r, c));
            }
        }
    }

    let mut accum = 1u64;
    for slope in &[(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)] {
        let mut trees = 0;
        let mut pos = (0, 0);
        while pos.0 < rows {
            pos = (pos.0 + slope.0, pos.1 + slope.1);
            if map.contains(&(pos.0, pos.1 % cols)) {
                trees += 1
            }
        }
        accum *= trees;
        dbg!(accum);
    }

    accum
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input));
    println!("{}", solve_p2(input));
}
