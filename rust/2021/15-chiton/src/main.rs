use std::cmp::max;
use std::collections::{BinaryHeap, HashMap};

// Point coordinate
type P = (i32, i32);
type Map = HashMap<P, u32>;

#[derive(PartialEq, Eq)]
struct ScoredNode {
    score: u32,
    p: P,
}

impl Ord for ScoredNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

impl PartialOrd for ScoredNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn distance(a: &P, b: &P) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

impl ScoredNode {
    fn new(score: u32, p: P) -> ScoredNode {
        ScoredNode { score, p }
    }
    fn distance(&self, other: &P) -> i32 {
        distance(&self.p, other)
    }
}

fn search(map: &Map, start: P, goal: P) -> u32 {
    let mut queue = BinaryHeap::<ScoredNode>::new();
    let mut total_risk_at = HashMap::<P, u32>::new();

    total_risk_at.insert(start, 0);
    queue.push(ScoredNode::new(map[&start], start));

    while let Some(current) = queue.pop() {
        if current.distance(&goal) == 0 {
            return total_risk_at[&goal];
        }

        let (r, c) = current.p;
        for next in [(r + 1, c), (r, c - 1), (r - 1, c), (r, c + 1)] {
            if let Some(risk_next) = map.get(&next) {
                let risk = total_risk_at[&current.p] + risk_next;
                let e = total_risk_at.entry(next).or_insert(u32::MAX);
                if risk < *e {
                    *e = risk;
                    queue.push(ScoredNode::new(risk + distance(&goal, &next) as u32, next));
                }
            }
        }
    }
    panic!("Did not find any path");
}

fn solve_p1(input: &str) -> u32 {
    let mut map = Map::new();
    let mut exit = (0, 0);
    for (r, line) in input.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            map.insert((r as i32, c as i32), ch.to_digit(10).unwrap() as u32);
            exit = (max(r as i32, exit.0), max(c as i32, exit.1));
        }
    }

    search(&map, (0, 0), exit)
}

fn solve_p2(input: &str) -> u32 {
    let mut tile = Map::new();
    let mut tile_rows = 0;
    let mut tile_cols = 0;
    for (r, line) in input.lines().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            let _new = tile.insert((r as i32, c as i32), ch.to_digit(10).unwrap() as u32);
            assert!(_new.is_none());
            tile_rows = max(r as i32, tile_rows);
            tile_cols = max(c as i32, tile_cols);
        }
    }
    tile_rows += 1;
    tile_cols += 1;

    let mut map = Map::new();
    for tile_row in 0..5 {
        for tile_col in 0..5 {
            for ((r, c), risk_inital) in &tile {
                let mut risk = risk_inital + tile_row as u32 + tile_col as u32;
                if risk >= 10 {
                    risk -= 9;
                }
                map.insert((tile_row * tile_rows + r, tile_col * tile_cols + c), risk);
            }
        }
    }

    let exit = (tile_rows * 5 - 1, tile_cols * 5 - 1);
    search(&map, (0, 0), exit)
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve_p1(input));
    println!("{}", solve_p2(input));
}

#[test]
fn test_p1() {
    let input = include_str!("input-simple");
    assert_eq!(solve_p1(input), 40);
}

#[test]
fn test_p2() {
    let input = include_str!("input-simple");
    assert_eq!(solve_p2(input), 315);
}
