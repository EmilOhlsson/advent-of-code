extern crate coding_challenge_utils as chs;

use chs::coord::Cartesian;
use std::collections::HashMap;
use std::collections::HashSet;

fn solve(input: &str, max_dist: usize) -> (usize, usize) {
    let mut coords = input
        .lines()
        .map(|l| (l.parse::<Cartesian>().unwrap(), 0))
        .collect::<HashMap<Cartesian, usize>>();

    let mut x_min = std::i32::MAX;
    let mut x_max = std::i32::MIN;
    let mut y_min = std::i32::MAX;
    let mut y_max = std::i32::MIN;
    for c in coords.keys() {
        if c.x < x_min {
            x_min = c.x;
        }
        if c.x > x_max {
            x_max = c.x;
        }
        if c.y < y_min {
            y_min = c.y;
        }
        if c.y > y_max {
            y_max = c.y;
        }
    }

    let mut grid: HashMap<Cartesian, Option<Cartesian>> = HashMap::new();

    println!("{},{}  {},{}", x_min, y_min, x_max, y_max);
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            let mut dist = std::usize::MAX;
            let mut closest: Option<Cartesian> = None;
            let t = Cartesian::new(x, y);
            for c in coords.keys() {
                let d = c.manhattan_distance(&t);
                if d < dist {
                    closest = Some(c.clone());
                    dist = d;
                } else if d == dist {
                    closest = None;
                }
            }
            grid.insert(t, closest);
        }
    }

    let mut infinite: HashSet<Cartesian> = HashSet::new();
    for x in x_min..=x_max {
        let top = Cartesian::new(x, y_min);
        let bot = Cartesian::new(x, y_max);
        if let Some(sov) = grid.get(&top).unwrap() {
            infinite.insert(sov.clone());
        }
        if let Some(sov) = grid.get(&bot).unwrap() {
            infinite.insert(sov.clone());
        }
    }
    for y in y_min..=y_max {
        let left = Cartesian::new(x_min, y);
        let right = Cartesian::new(x_max, y);
        if let Some(sov) = grid.get(&left).unwrap() {
            infinite.insert(sov.clone());
        }
        if let Some(sov) = grid.get(&right).unwrap() {
            infinite.insert(sov.clone());
        }
    }

    println!("There are {} inf", infinite.len());
    for x in x_min..=x_max {
        for y in y_min..=y_max {
            let t = Cartesian::new(x, y);
            if let Some(closest) = grid.get(&t) {
                if let Some(sov) = closest {
                    let count: &mut usize = coords.get_mut(sov).unwrap();
                    *count += 1;
                }
            } else {
                panic!("Unable to get coord");
            }
        }
    }

    let mut safe_zone = 0;
    for y in y_min..=y_max {
        for x in x_min..=x_max {
            let t = Cartesian::new(x, y);
            if coords
                .keys()
                .map(|c| c.manhattan_distance(&t))
                .sum::<usize>()
                < max_dist
            {
                safe_zone += 1;
            }
        }
    }

    (
        coords
            .iter()
            .filter_map(|(c, size)| {
                if infinite.contains(c) {
                    None
                } else {
                    Some(*size)
                }
            }).max()
            .unwrap(),
        safe_zone,
    )
}

fn main() {
    let input = include_str!("input");
    let solution = solve(input, 10000);
    println!("{}, {}", solution.0, solution.1);
}

#[test]
fn test() {
    let input = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";
    assert_eq!(solve(input, 32), (17, 16));
}
