use std::collections::{HashMap, HashSet};

fn solve(input: &str, part1: bool) -> (u32, i32) {
    let mut asteroids = HashSet::new();
    for (y, line) in input.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                asteroids.insert((x as i32, y as i32));
            }
        }
    }

    let mut candidates = HashMap::new();
    for asteroid in &asteroids {
        let mut angles = HashSet::new();
        for other in &asteroids {
            if asteroid == other {
                continue;
            }
            let dx = (other.0 - asteroid.0) as f64;
            let dy = (other.1 - asteroid.1) as f64;
            let angle = (dy.atan2(dx) * 10_000f64).round() as u32;
            angles.insert(angle);
        }
        candidates.insert(asteroid, angles.len() as u32);
    }
    let sol = candidates.iter().max_by_key(|&(_, n)| n).unwrap();
    println!("Solution: {:?}", sol);
    if part1 {
        return (*sol.1, 0);
    }

    // Part two
    // Create map of angles and asteroids in that angle
    let station = *sol.0;
    let mut targets: HashMap<i32, Vec<(i32, i32)>> = HashMap::new();
    for other in &asteroids {
        if station == other {
            continue;
        }
        let dx = (station.0 - other.0) as f64;
        let dy = (station.1 - other.1) as f64;
        let angle = (dy.atan2(dx) / std::f64::consts::PI * 180_000f64).round() as i32 - 90_000;
        let angle_tanslated = if angle >= 0 { angle } else { angle + 360_000 };

        let target = targets.entry(angle_tanslated).or_insert_with(Vec::new);
        target.push(*other);
    }

    // Make sure all lists are sorted by distance from station
    for (_, ast) in targets.iter_mut() {
        ast.sort_unstable_by_key(|(x, y)| {
            let dx = x - station.0;
            let dy = y - station.1;
            dx * dx + dy * dy
        });
        ast.reverse();
    }

    let mut angles = targets.keys().cloned().collect::<Vec<i32>>();
    angles.sort_unstable();
    let mut destroyed = Vec::new();
    for angle in angles.iter().cycle() {
        let targeted = targets.get_mut(angle).unwrap();
        if let Some(destr) = targeted.pop() {
            println!("Destroying; {:?}@{}", destr, angle / 1_000);
            destroyed.push(destr);
            if destroyed.len() >= 200 {
                break;
            }
        }
    }
    println!("Solution: {:?}", destroyed[199]);
    let sol2 = destroyed[199].0 * 100 + destroyed[199].1;

    (*sol.1, sol2)
}

fn main() {
    let input = include_str!("input");
    println!("{:?}", solve(input, false));
}

#[test]
fn test0() {
    assert_eq!(solve(include_str!("input-test0"), true).0, 33);
}
#[test]
fn test1() {
    assert_eq!(solve(include_str!("input-test1"), true).0, 35);
}
#[test]
fn test2() {
    assert_eq!(solve(include_str!("input-test2"), true).0, 41);
}
#[test]
fn test3() {
    assert_eq!(solve(include_str!("input-test3"), false), (210, 802));
}
