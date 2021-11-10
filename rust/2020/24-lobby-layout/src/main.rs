use std::collections::{HashMap, HashSet};

fn adjacents(pos: (i32, i32, i32)) -> [(i32, i32, i32); 6] {
    [
        (pos.0 + 1, pos.1, pos.2 - 1),
        (pos.0 + 1, pos.1 - 1, pos.2),
        (pos.0, pos.1 - 1, pos.2 + 1),
        (pos.0, pos.1 + 1, pos.2 - 1),
        (pos.0 - 1, pos.1 + 1, pos.2),
        (pos.0 - 1, pos.1, pos.2 + 1),
    ]
}

fn living_floor(mut black_tiles: HashSet<(i32, i32, i32)>) -> usize {
    for _day in 1..=100 {
        let white_candidates = black_tiles.clone();
        let mut black_candidates = HashSet::new();
        for pos in &white_candidates {
            for adj in &adjacents(*pos) {
                if !white_candidates.contains(adj) {
                    black_candidates.insert(*adj);
                }
            }
        }

        let mut black_tiles_new = HashSet::new();
        // flip white
        for tile in &white_candidates {
            let count = adjacents(*tile)
                .iter()
                .filter(|adj| black_tiles.contains(adj))
                .count();
            if count == 1 || count == 2 {
                black_tiles_new.insert(*tile);
            }
        }

        // Flip black
        for tile in &black_candidates {
            let count = adjacents(*tile)
                .iter()
                .filter(|adj| black_tiles.contains(adj))
                .count();
            if count == 2 {
                black_tiles_new.insert(*tile);
            }
        }
        black_tiles = black_tiles_new;
    }
    black_tiles.len()
}

fn solve(input: &str) -> (usize, usize) {
    let mut tiles = HashMap::<(i32, i32, i32), bool>::new();
    for line in input.lines() {
        let mut pos = (0i32, 0i32, 0i32);
        let mut prev = None;

        for ch in line.chars() {
            match ch {
                'e' => {
                    if prev == Some('n') {
                        pos = (pos.0 + 1, pos.1, pos.2 - 1);
                    } else if prev == Some('s') {
                        pos = (pos.0, pos.1 - 1, pos.2 + 1);
                    } else {
                        pos = (pos.0 + 1, pos.1 - 1, pos.2);
                    }
                    prev = None;
                }
                'w' => {
                    if prev == Some('n') {
                        pos = (pos.0, pos.1 + 1, pos.2 - 1);
                    } else if prev == Some('s') {
                        pos = (pos.0 - 1, pos.1, pos.2 + 1);
                    } else {
                        pos = (pos.0 - 1, pos.1 + 1, pos.2);
                    }
                    prev = None;
                }
                'n' | 's' => prev = Some(ch),
                _ => panic!(),
            }
        }
        let tile = tiles.entry(pos).or_insert(false);
        *tile = !*tile
    }
    let part1 = tiles.values().filter(|&black| *black).count();
    let black_tiles = tiles
        .iter()
        .filter_map(|(pos, black)| if *black { Some(*pos) } else { None })
        .collect::<HashSet<(i32, i32, i32)>>();
    let part2 = living_floor(black_tiles);

    (part1, part2)
}

fn main() {
    let input = include_str!("input");
    println!("{:?}", solve(input));
}

#[test]
fn test() {
    let input = include_str!("input-simple");
    assert_eq!(solve(input), (10, 2208));
}
