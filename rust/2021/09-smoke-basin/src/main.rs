use std::collections::{HashMap, HashSet};

fn get_basin_size(map: &HashMap<(i32, i32), u32>, (r, c): &(i32, i32)) -> usize {
    let mut visited = HashSet::new();
    fn flow_from(
        map: &HashMap<(i32, i32), u32>,
        visited: &mut HashSet<(i32, i32)>,
        r: i32,
        c: i32,
    ) {
        if visited.insert((r, c)) {
            let h = map[&(r, c)];
            for (dr, dc) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
                let &h_n = map.get(&(r + dr, c + dc)).unwrap_or(&9);
                if h_n != 9 && h_n > h {
                    flow_from(map, visited, r + dr, c + dc);
                }
            }
        }
    }
    flow_from(map, &mut visited, *r, *c);
    visited.len()
}

fn solve(input: &str) -> (u32, u32) {
    let map = input
        .lines()
        .enumerate()
        .map(move |(r, line)| {
            line.chars()
                .enumerate()
                .map(move |(c, ch)| ((r as i32, c as i32), ch.to_digit(10).unwrap()))
        })
        .flatten()
        .collect::<HashMap<(i32, i32), u32>>();

    let low_points = map.iter().filter(|((r, c), &h)| {
        [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .iter()
            .all(|(dr, dc)| h < *map.get(&(r + dr, c + dc)).unwrap_or(&10))
    });

    let part1 = low_points.clone().map(|((_, _), h)| h + 1).sum();

    let mut basins = low_points
        .map(|(p, _)| get_basin_size(&map, p))
        .collect::<Vec<usize>>();
    basins.sort_unstable();
    let part2 = basins.iter().rev().take(3).product::<usize>() as u32;

    (part1, part2)
}

fn main() {
    let input = include_str!("input");
    println!("{:?}", solve(input));
}
