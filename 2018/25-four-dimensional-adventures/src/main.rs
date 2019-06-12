use std::collections::HashSet;

type Point = Vec<i32>;

fn dist(v1: &[i32], v2: &[i32]) -> i32 {
    v1.iter().zip(v2.iter()).map(|(a, b)| (a - b).abs()).sum()
}

fn build(input: &str) -> Vec<Point> {
    input
        .lines()
        .map(|line| {
            line.split(',')
                .map(|tok| tok.parse::<i32>().unwrap())
                .collect::<Vec<i32>>()
        })
        .collect()
}

fn dfs_visit(visited: &mut HashSet<Point>, p: &Point, ps: &[Point]) {
    visited.insert(p.clone());
    for pnext in ps.iter().filter(|pn| dist(p, pn) <= 3) {
        if !visited.contains(pnext) {
            dfs_visit(visited, pnext, ps);
        }
    }
}

fn solve(input: &str) -> i32 {
    let mut ps: Vec<Point> = build(input);
    let mut visited: HashSet<Point> = HashSet::new();

    let mut count = 0;
    while let Some(p) = ps.pop() {
        if visited.insert(p.clone()) {
            dfs_visit(&mut visited, &p, &ps);
            count += 1;
        }
    }

    count
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input));
}

#[test]
fn test1() {
    assert_eq!(solve(include_str!("input-simple-1")), 2);
}

#[test]
fn test2() {
    assert_eq!(solve(include_str!("input-simple-2")), 4);
}

#[test]
fn test3() {
    assert_eq!(solve(include_str!("input-simple-3")), 3);
}

#[test]
fn test4() {
    assert_eq!(solve(include_str!("input-simple-4")), 8);
}
