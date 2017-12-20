use std::str::FromStr;
use std::num::ParseIntError;
use std::ops::Add;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
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

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

#[derive(Copy, Clone)]
struct State {
    p: Vector,
    v: Vector,
    a: Vector,
}

impl State {
    fn tick(&mut self) {
        self.v = self.v.clone() + self.a.clone();
        self.p = self.p.clone() + self.v.clone();
    }
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

fn collide(input: &str) -> usize {
    let mut ps: Vec<State> = input
        .lines()
        .map(|l| l.parse::<State>().unwrap())
        .collect::<Vec<State>>();
    for _ in 0..1_000_000 {
        ps.iter_mut().for_each(|ref mut p| p.tick());
        ps.sort_by_key(|p| p.p);
        let mut i = 1;
        let mut v_prev = ps[0].clone();
        let mut remove_prev = true;
        while let Some(&p) = ps.get(i) {
            if p.p == v_prev.p {
                ps.remove(i);
                if remove_prev {
                   ps.remove(i - 1);
                   remove_prev = false;
                   i = i.saturating_sub(1);
                }
            } else {
                v_prev = p.clone();
                i += 1;
                remove_prev = true;
            }
        }
    }

    return ps.len();
}

fn main() {
    let input = include_str!("input");
    println!("{}", collide(input));
}

#[test]
fn test_collision_removal() {
    let mut vs = vec![5, 5, 5, 5, 5, 5, 10, 15, 15, 15, 20, 25, 25, 30, 35, 35];
    let mut i = 1;
    let mut v_prev = vs[0].clone();
    let mut remove_prev = true;

    while let Some(&v) = vs.get(i) {
        if v == v_prev {
            vs.remove(i);
            if remove_prev {
               vs.remove(i - 1);
               remove_prev = false;
               i = i.saturating_sub(1);
            }
        } else {
            v_prev = v.clone();
            i += 1;
            remove_prev = true;
        }
    }
    assert_eq!(vs, vec![10, 20, 30]);
}
