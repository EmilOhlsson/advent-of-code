use std::collections::HashSet;

fn solve_p1(input: &str) -> u32 {
    let mut pos: (i32, i32) = (0, 0);
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    visited.insert(pos);
    for c in input.chars() {
        match c {
            '<' => pos.0 -= 1,
            '>' => pos.0 += 1,
            'v' => pos.1 += 1,
            '^' => pos.1 -= 1,
            _ => panic!("I don't know what to do with {:?}", c),
        }
        visited.insert(pos);
    }

    visited.len() as u32
}

fn solve_p2(input: &str) -> u32 {
    let mut pos: [(i32, i32); 2] = [(0, 0), (0, 0)];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    visited.insert(pos[0]);
    visited.insert(pos[1]);
    for (i, c) in input.chars().enumerate() {
        let sel = i & 1;
        match c {
            '<' => pos[sel].0 -= 1,
            '>' => pos[sel].0 += 1,
            'v' => pos[sel].1 += 1,
            '^' => pos[sel].1 -= 1,
            _ => panic!("I don't know what to do with {:?}", c),
        }
        visited.insert(pos[sel]);
    }

    visited.len() as u32
}

fn main() {
    let input = include_str!("input");
    println!("p1: {}", solve_p1(input.to_string().trim()));
    println!("p2: {}", solve_p2(input.to_string().trim()));
}

#[test]
fn test_p1() {
    let input = "^v^v^v^v^v";
    assert_eq!(solve_p1(input.to_string().trim()), 2);
}

#[test]
fn test_p2() {
    let input = "^v^v^v^v^v";
    assert_eq!(solve_p2(input.to_string().trim()), 11);
}
