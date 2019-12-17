pub mod intmachine;

use intmachine::Intmachine;
use std::collections::HashSet;

fn around(map: &HashSet<(i32, i32)>, (x, y): (i32, i32)) -> bool {
    map.contains(&(x - 1, y))
        && map.contains(&(x, y - 1))
        && map.contains(&(x + 1, y))
        && map.contains(&(x, y + 1))
}

fn find_junctions(map_rep: &str) -> i32 {
    let mut map = std::collections::HashSet::<(i32, i32)>::new();
    for (y, line) in map_rep.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                map.insert((x as i32, y as i32));
            }
        }
    }

    let mut sum = 0;
    for p in &map {
        if around(&map, *p) {
            sum += p.0 * p.1;
        }
    }
    sum
}

fn solve(input: &str) -> i32 {
    let mut ascii = Intmachine::load(input);
    let out = ascii.run([].iter());
    let out_rep = out.iter().map(|&v| (v as u8) as char).collect::<String>();
    println!("{}", out_rep);
    find_junctions(&out_rep)
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input));
}

#[test]
fn test() {
    let input = include_str!("input-simple-map");
    assert_eq!(find_junctions(input), 76);
}
