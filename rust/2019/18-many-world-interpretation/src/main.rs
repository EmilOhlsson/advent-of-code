use std::cmp::{max, min, Ordering};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};

type Pos = (i32, i32);
type Maze = HashSet<Pos>;
type Trace = HashMap<Pos, Pos>;

/// Mapping lookup distance to key, and keys required to get there
type DMap = HashMap<char, (u32, HashSet<char>)>;

#[derive(Debug)]
struct Labyrinth {
    maze: Maze,
    starts: Vec<Pos>,
    keys: HashMap<char, Pos>,
    doors: HashMap<Pos, char>,
}

impl Labyrinth {
    fn build(input: &str) -> Labyrinth {
        let mut maze = HashSet::new();
        let mut keys = HashMap::new();
        let mut doors = HashMap::new();
        let mut starts = Vec::new();

        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let p = (x as i32, y as i32);

                match ch {
                    '@' => {
                        starts.push(p);
                    }
                    key @ 'a'..='z' => {
                        keys.insert(key, p);
                    }

                    door @ 'A'..='Z' => {
                        doors.insert(p, door.to_ascii_lowercase());
                    }
                    _ => (),
                }
                if ch != '#' {
                    maze.insert(p);
                }
            }
        }

        Labyrinth {
            maze,
            starts,
            keys,
            doors,
        }
    }

    fn bfs(&self, pos: Pos) -> DMap {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let mut came_from = Trace::new();

        visited.insert(pos);
        queue.push_back(pos);
        while !queue.is_empty() {
            let p = queue.pop_front().unwrap();
            visited.insert(p);

            for pn in &[
                (p.0 - 1, p.1),
                (p.0, p.1 - 1),
                (p.0 + 1, p.1),
                (p.0, p.1 + 1),
            ] {
                if self.maze.contains(pn) && !visited.contains(pn) {
                    came_from.insert(*pn, p);
                    queue.push_back(*pn);
                }
            }
        }
        self.backtrack(&came_from)
    }

    fn backtrack(&self, came_from: &Trace) -> DMap {
        let mut endpoints = HashMap::<char, (u32, HashSet<char>)>::new();
        for (key, pos) in &self.keys {
            if came_from.get(pos).is_none() {
                continue;
            }
            let mut distance = 0;
            let mut current = pos;
            let mut keys_needed = HashSet::new();
            while let Some(prev) = came_from.get(current) {
                distance += 1;
                current = prev;
                if let Some(k) = self.doors.get(&current) {
                    keys_needed.insert(*k);
                }
            }
            endpoints.insert(*key, (distance, keys_needed));
        }
        endpoints
    }
}

#[derive(Debug)]
struct MazeSolver {
    starts: Vec<(Pos, DMap)>,
    key_traces: HashMap<char, DMap>,
    keys_needed: usize,
    key_positions: HashMap<char, Pos>,
    labyrinth: Labyrinth,
}

#[derive(Debug, Eq, PartialEq)]
struct DNode<'a> {
    pos: Pos,
    dist: u32,
    dmap: &'a DMap,
    keys: HashSet<char>,
}

impl<'a> Ord for DNode<'a> {
    fn cmp(&self, other: &DNode<'a>) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl<'a> PartialOrd for DNode<'a> {
    fn partial_cmp(&self, other: &DNode<'a>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn keyset_to_u32(keys: &HashSet<char>) -> u32 {
    keys.iter()
        .map(|&ch| 1 << (ch as u8 - b'a'))
        .fold(0, |acc, v| v | acc)
}

fn dfilt(
    (key, (dist, keys_needed)): &(&char, &(u32, HashSet<char>)),
    keyset: &HashSet<char>,
) -> Option<(char, u32)> {
    if !keyset.contains(key) && keys_needed.is_subset(keyset) {
        Some((**key, *dist))
    } else {
        None
    }
}

impl MazeSolver {
    fn solve(&self, trace: &DMap, starting_keys: HashSet<char>) -> u32 {
        let mut queue = BinaryHeap::new();
        queue.push(DNode {
            pos: (0, 0),
            dist: 0,
            dmap: trace,
            keys: starting_keys,
        });
        let mut seen = HashSet::new();

        while let Some(dnode) = queue.pop() {
            if seen.insert((dnode.pos, keyset_to_u32(&dnode.keys))) {
                if dnode.keys.len() == self.keys_needed {
                    return dnode.dist;
                }
                for (key, d) in dnode.dmap.iter().filter_map(|d| dfilt(&d, &dnode.keys)) {
                    let mut keyset_new = dnode.keys.clone();
                    keyset_new.insert(key);
                    queue.push(DNode {
                        pos: self.key_positions[&key],
                        dist: dnode.dist + d,
                        dmap: &self.key_traces[&key],
                        keys: keyset_new,
                    });
                }
            } else {
            }
        }

        panic!("Did not find a solution");
    }

    fn build(input: &str) -> MazeSolver {
        let labyrinth = Labyrinth::build(input);
        let starts = labyrinth
            .starts
            .iter()
            .map(|&start| (start, labyrinth.bfs(start)))
            .collect::<Vec<_>>();
        let key_traces = labyrinth
            .keys
            .iter()
            .map(|(k, p)| (*k, labyrinth.bfs(*p)))
            .collect::<HashMap<char, DMap>>();
        MazeSolver {
            starts,
            key_traces,
            keys_needed: labyrinth.keys.keys().len(),
            key_positions: labyrinth.keys.clone(),
            labyrinth,
        }
    }
}

fn solve(input: &str) -> u32 {
    let solver = MazeSolver::build(input);
    solver.solve(&solver.starts[0].1, HashSet::new())
}

fn in_range(v: i32, a: i32, b: i32) -> bool {
    let lo = min(a, b);
    let hi = max(a, b);
    lo <= v && v <= hi
}

fn in_box(p: (i32, i32), a: (i32, i32), b: (i32, i32)) -> bool {
    let in_x = in_range(p.0, a.0, b.0);
    let in_y = in_range(p.1, a.1, b.1);
    in_x && in_y
}

fn solve_v2(input: &str) -> u32 {
    let solver = MazeSolver::build(input);
    let mut sum = 0;
    let inf = 1000;
    let quadrants = [
        ((0, 0), (solver.starts[0].0)),
        ((solver.starts[1].0), (inf, 0)),
        ((0, inf), (solver.starts[2].0)),
        (solver.starts[3].0, (inf, inf)),
    ];
    for (i, (_, trace)) in solver.starts.iter().enumerate() {
        // Build a keyset of all keys that aren't in the current quadrant
        let keys = solver
            .labyrinth
            .keys
            .iter()
            .filter_map(|(key, key_pos)| {
                if !in_box(*key_pos, quadrants[i].0, quadrants[i].1) {
                    Some(*key)
                } else {
                    None
                }
            })
            .collect::<HashSet<char>>();
        sum += solver.solve(trace, keys)
    }
    sum
}

fn main() {
    let input_p1 = include_str!("input-p1");
    let input_p2 = include_str!("input-p2");
    println!("{}", solve(input_p1));
    println!("{}", solve_v2(input_p2));
}

#[test]
fn test_simple() {
    assert_eq!(solve(include_str!("input-simple")), 8);
}

#[test]
fn test_simple2() {
    assert_eq!(solve_v2(include_str!("input-simple-p2")), 8);
}

#[test]
fn test0() {
    assert_eq!(solve(include_str!("input-test0")), 86);
}

#[test]
fn test1() {
    assert_eq!(solve(include_str!("input-test1")), 132);
}

#[test]
fn test2() {
    assert_eq!(solve(include_str!("input-test2")), 136);
}

#[test]
fn test3() {
    assert_eq!(solve(include_str!("input-test3")), 81);
}
