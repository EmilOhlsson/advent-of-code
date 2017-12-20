use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Clone, Copy)]
struct Vector {
    x: isize,
    y: isize,
    z: isize,
}

impl FromStr for Vector {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.split('=')
            .skip(1)
            .map(|t| {
                t.trim_matches(|c| c == '<' || c == '>')
                    .split(',')
                    .map(|n| n.parse::<isize>().unwrap())
                    .collect::<Vec<isize>>()
            })
            .next()
            .unwrap();
        return Ok(Vector {
            x: v[0],
            y: v[1],
            z: v[2],
        });
    }
}

impl Vector {
    fn abs(&self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

struct State {
    p: Vector,
    v: Vector,
    a: Vector,
}

impl FromStr for State {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v = s.split(", ")
            .map(|t| t.parse::<Vector>().unwrap())
            .collect::<Vec<Vector>>();
        return Ok(State {
            p: v[0].clone(),
            v: v[1].clone(),
            a: v[2].clone(),
        });
    }
}

fn closest(input: &str) -> usize {
    input
        .lines()
        .enumerate()
        .map(|(i, l)| (i, l.parse::<State>().unwrap().a.abs()))
        .min_by_key(|&(_, v)| v)
        .unwrap()
        .0
}

fn main() {
    let input = include_str!("input");
    println!("{}", closest(input));
}

#[test]
fn test_closest() {
    let input = include_str!("input-simple");
    assert_eq!(closest(input), 0);
}
