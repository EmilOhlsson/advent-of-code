pub mod intmachine;

use intmachine::Intmachine;
use std::collections::HashSet;

fn around(map: &HashSet<(i32, i32)>, (x, y): (i32, i32)) -> bool {
    map.contains(&(x - 1, y))
        && map.contains(&(x, y - 1))
        && map.contains(&(x + 1, y))
        && map.contains(&(x, y + 1))
}

fn get_map_and_pos(map_rep: &str) -> (HashSet<(i32, i32)>, (i32, i32)) {
    let mut map = HashSet::<(i32, i32)>::new();
    let mut pos = (0, 0);
    for (y, line) in map_rep.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == '#' {
                map.insert((x as i32, y as i32));
            } else if ch == '^' {
                map.insert((x as i32, y as i32));
                pos = (x as i32, y as i32);
            }
        }
    }
    (map, pos)
}

fn find_junctions(map_rep: &str) -> i32 {
    let map = get_map_and_pos(map_rep).0;
    let mut sum = 0;
    for p in &map {
        if around(&map, *p) {
            sum += p.0 * p.1;
        }
    }
    sum
}

fn get_map(input: &str) -> String {
    let mut ascii = Intmachine::load(input);
    let out = ascii.run([].iter());
    out.iter().map(|&v| (v as u8) as char).collect::<String>()
}

fn solve(input: &str) -> i32 {
    let out_rep = get_map(input);
    println!("{}", out_rep);
    find_junctions(&out_rep)
}

/// Find turn direction (and name) that allows you move further.
/// If turn doesnt help, return None
fn get_turn(
    (x, y): (i32, i32),
    (dx, dy): (i32, i32),
    map: &HashSet<(i32, i32)>,
) -> Option<(&str, (i32, i32))> {
    let left = (dy, -dx);
    let right = (-dy, dx);
    if map.contains(&(x + left.0, y + left.1)) {
        Some(("L", left))
    } else if map.contains(&(x + right.0, y + right.1)) {
        Some(("R", right))
    } else {
        None
    }
}

fn build_directions(map_str: &str) -> Vec<String> {
    let mut instructions = Vec::<String>::new();
    let (map, mut pos) = get_map_and_pos(&map_str);
    let mut dir = (0, -1);

    // If any, get valid turn, move as far as you can, repeat until there is no
    // valid turn anymore
    while let Some((t, new_dir)) = get_turn(pos, dir, &map) {
        dir = new_dir;
        instructions.push(t.to_string());
        let mut steps = 0;
        while map.contains(&(pos.0 + dir.0, pos.1 + dir.1)) {
            pos = (pos.0 + dir.0, pos.1 + dir.1);
            steps += 1;
        }
        instructions.push(format!("{}", steps));
    }

    instructions
}

fn solve_v2(input: &str) -> i32 {
    let map_str = get_map(input);
    let directions = build_directions(&map_str);
    let dir_rep = directions.join(",");

    println!("{:?}", dir_rep);
    // Used vim to search for repeats. Programmatic solution would require
    // searching and finding a lot of matching substrings, and then get the
    // program order correct.
    let prog_order = "A,B,A,B,C,C,B,C,B,A\n";
    let a_prog = "R,12,L,8,R,12\n";
    let b_prog = "R,8,R,6,R,6,R,8\n";
    let c_prog = "R,8,L,8,R,8,R,4,R,4\n";

    // Transform program listings to input, don't need debug output.
    let prog_input = [prog_order, a_prog, b_prog, c_prog, "n\n"]
        .iter()
        .map(|s| s.chars())
        .flatten()
        .map(|ch| ch as i64)
        .collect::<Vec<i64>>();
    let mut ascii = Intmachine::load(input);
    ascii.set_addr(0, 2);
    let out = ascii.run(prog_input.iter());
    *out.last().unwrap() as i32
}

fn main() {
    let input = include_str!("input");
    println!("part1: {}", solve(input));
    println!("part2: {}", solve_v2(input));
}

#[test]
fn test() {
    let input = include_str!("input-simple-map");
    assert_eq!(find_junctions(input), 76);
}
