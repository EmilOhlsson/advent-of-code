use std::collections::{HashMap, HashSet, VecDeque};

type Map = HashMap<(i32, i32), u32>;
type Pos = HashSet<(i32, i32)>;

static AROUND: [(i32, i32); 8] = [
    (1, 0),
    (1, 1),
    (0, 1),
    (-1, 1),
    (-1, 0),
    (-1, -1),
    (0, -1),
    (1, -1),
];

fn tick(octopi: &mut Map) -> usize {
    // Increase generation
    octopi.values_mut().for_each(|v| *v += 1);

    // Gather all starting points for flashes
    let mut sp = octopi
        .iter()
        .filter_map(|(p, v)| if *v >= 10 { Some(*p) } else { None })
        .collect::<VecDeque<(i32, i32)>>();

    // Visit all flashing points, and increase the neigboring points
    // of those. Keep track of all points that already flash to not
    // revisit those.
    let mut flashed = Pos::new();
    while let Some((r, c)) = sp.pop_front() {
        if flashed.insert((r, c)) {
            for (dr, dc) in AROUND {
                let p = (r + dr, c + dc);
                if let Some(v) = octopi.get_mut(&p) {
                    *v += 1;
                    if *v >= 10 {
                        sp.push_back(p);
                    }
                }
            }
        }
    }

    // Clamp down to zero
    flashed.iter().for_each(|p| {
        *octopi.get_mut(p).unwrap() = 0;
    });

    flashed.len()
}

fn solve(input: &str) -> (usize, usize) {
    let mut octopi = input
        .lines()
        .enumerate()
        .map(move |(row, line)| {
            line.chars()
                .enumerate()
                .map(move |(col, char)| ((row as i32, col as i32), char.to_digit(10).unwrap()))
        })
        .flatten()
        .collect::<Map>();
    let size = octopi.len();

    let mut part1 = 0;
    for _ in 1..=100 {
        part1 += tick(&mut octopi);
    }

    let mut part2 = None;
    for step in 101.. {
        if tick(&mut octopi) == size {
            part2 = Some(step);
            break;
        }
    }

    (part1, part2.unwrap())
}

fn main() {
    let input = include_str!("input");
    println!("{:?}", solve(input));
}

#[test]
fn test() {
    let input = include_str!("input-simple");
    assert_eq!(solve(input), (1656, 195));
}
