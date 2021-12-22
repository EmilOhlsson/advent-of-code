use regex::Regex;
use std::cmp::{max, min};
use std::collections::HashSet;

type Xy = (i64, i64);
type Xyz = (i64, i64, i64);
type Cube = (Xy, Xy, Xy);
type Reactors = HashSet<Xyz>;

fn solve_p1(input: &str) -> usize {
    let mut reactors = Reactors::new();
    let re =
        Regex::new(r"^(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)$").unwrap();

    for cap in input.lines().map(|l| re.captures(l).unwrap()) {
        let (x_lo, x_hi) = (
            cap[2].parse::<i64>().unwrap(),
            cap[3].parse::<i64>().unwrap(),
        );
        let (y_lo, y_hi) = (
            cap[4].parse::<i64>().unwrap(),
            cap[5].parse::<i64>().unwrap(),
        );
        let (z_lo, z_hi) = (
            cap[6].parse::<i64>().unwrap(),
            cap[7].parse::<i64>().unwrap(),
        );

        for x in max(x_lo, -50)..=min(x_hi, 50) {
            for y in max(y_lo, -50)..=min(y_hi, 50) {
                for z in max(z_lo, -50)..=min(z_hi, 50) {
                    if &cap[1] == "on" {
                        reactors.insert((x, y, z));
                    } else {
                        reactors.remove(&(x, y, z));
                    }
                }
            }
        }
    }

    reactors.len()
}

enum Op {
    On,
    Off,
}

fn parse(input: &str) -> Vec<(Op, Cube)> {
    let re =
        Regex::new(r"^(on|off) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)$").unwrap();

    input
        .lines()
        .map(|l| re.captures(l).unwrap())
        .map(|cap| {
            let cube = (
                (
                    cap[2].parse::<i64>().unwrap(),
                    cap[3].parse::<i64>().unwrap(),
                ),
                (
                    cap[4].parse::<i64>().unwrap(),
                    cap[5].parse::<i64>().unwrap(),
                ),
                (
                    cap[6].parse::<i64>().unwrap(),
                    cap[7].parse::<i64>().unwrap(),
                ),
            );

            if &cap[1] == "on" {
                (Op::On, cube)
            } else {
                (Op::Off, cube)
            }
        })
        .collect()
}

fn intersection((xa, ya, za): &Cube, (xb, yb, zb): &Cube) -> Option<Cube> {
    let cube = (
        (max(xa.0, xb.0), min(xa.1, xb.1)),
        (max(ya.0, yb.0), min(ya.1, yb.1)),
        (max(za.0, zb.0), min(za.1, zb.1)),
    );
    let ((x_lo, x_hi), (y_lo, y_hi), (z_lo, z_hi)) = cube;
    if x_lo <= x_hi && y_lo <= y_hi && z_lo <= z_hi {
        Some(cube)
    } else {
        None
    }
}

fn volume(((x_lo, x_hi), (y_lo, y_hi), (z_lo, z_hi)): &Cube) -> i64 {
    (x_hi - x_lo + 1) * (y_hi - y_lo + 1) * (z_hi - z_lo + 1)
}

fn solve_p2(input: &str) -> i64 {
    let operations = parse(input);
    let mut cubes = Vec::new();
    for (op, cube) in &operations {
        let mut cubes_new = Vec::new();
        
        /* Turn on given new cube */
        if matches!(op, Op::On) {
            cubes_new.push((1, *cube));
        }

        /* Create new intersecting cubes, with toggled signs */
        for (sign, intersecting) in cubes
            .iter()
            .filter_map(|(s, c)| intersection(c, cube).map(|c| (s, c)))
        {
            cubes_new.push((-sign, intersecting));
        }
        cubes.append(&mut cubes_new);
    }
    cubes.iter().map(|(s, c)| s * volume(c)).sum()
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve_p1(input));
    println!("{}", solve_p2(input));
}

#[test]
fn simple() {
    let input = include_str!("input-simple");
    assert_eq!(solve_p1(input), 590784);
}

#[test]
fn trivial() {
    let input = include_str!("input-trivial");
    assert_eq!(solve_p1(input), 39);
}

#[test]
fn part2_trivial() {
    let input = include_str!("input-trivial");
    assert_eq!(solve_p2(input), 39);
}

#[test]
fn part2() {
    let input = include_str!("input-part2");
    assert_eq!(solve_p2(input), 2758514936282235);
}
