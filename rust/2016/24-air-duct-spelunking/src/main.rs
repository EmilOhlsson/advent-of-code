use itertools::Itertools;
use std::collections::{HashMap, HashSet, VecDeque};

fn find_shortest(distances: &[Vec<u32>], nodes: &[u32], order: Vec<u32>, back: bool) -> u32 {
    if order.len() == nodes.len() {
        order
            .iter()
            .tuple_windows()
            .map(|(s, d)| distances[*s as usize][*d as usize])
            .sum::<u32>()
            + if back {
                distances[*order.last().unwrap() as usize][*order.first().unwrap() as usize]
            } else {
                0
            }
    } else {
        let mut min_distance = std::u32::MAX;
        for n in nodes.iter().filter(|n| !order.contains(n)) {
            let mut order_new = order.clone();
            order_new.push(*n);
            min_distance = std::cmp::min(
                find_shortest(distances, nodes, order_new, back),
                min_distance,
            );
        }
        min_distance
    }
}

fn solve(input: &str, back: bool) -> u32 {
    let mut map = HashSet::<(i32, i32)>::new();
    let mut pois = HashMap::<(i32, i32), u32>::new();

    // Parse
    for (r, c, ch) in input
        .lines()
        .enumerate()
        .map(|(r, line)| {
            line.chars()
                .enumerate()
                .map(move |(c, ch)| (r as i32, c as i32, ch))
        })
        .flatten()
    {
        if ch != '#' {
            map.insert((r, c));
        }
        if let Some(d) = ch.to_digit(10) {
            pois.insert((r, c), d);
        }
    }
    let npois = pois.len();
    let mut distances = vec![vec![0u32; npois]; npois];

    // build shotests distances
    for (source, src) in &pois {
        let mut enqued = HashSet::<(i32, i32)>::new();
        let mut visit_queue = VecDeque::<((i32, i32), u32)>::new();
        visit_queue.push_back((*source, 0));

        while let Some(((x, y), d)) = visit_queue.pop_front() {
            if let Some(poi) = pois.get(&(x, y)) {
                distances[*src as usize][*poi as usize] = d;
            }
            for (dx, dy) in &[(-1, 0), (1, 0), (0, -1), (0, 1)] {
                let point_next = (x + dx, y + dy);
                if map.contains(&point_next) && enqued.insert(point_next) {
                    visit_queue.push_back((point_next, d + 1));
                }
            }
        }
    }

    let nodes = pois.values().cloned().collect::<Vec<u32>>();
    find_shortest(&distances, &nodes, vec![0], back)
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input, false));
    println!("{}", solve(input, true));
}

#[test]
fn test() {
    let input = include_str!("input-simple");
    assert_eq!(solve(input, false), 14);
}
