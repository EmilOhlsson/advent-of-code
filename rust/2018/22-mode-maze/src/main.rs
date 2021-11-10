use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Terrain {
    Rocky,
    Wet,
    Narrow,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Gear {
    Neither,
    Climbing,
    Torch,
}

struct Maze {
    tiles: HashMap<(i32, i32), i32>,
    entry: (i32, i32),
    target: (i32, i32),
    depth: i32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Position {
    time: i32,
    pos: (i32, i32),
    gear: Gear,
}

impl Ord for Position {
    fn cmp(&self, other: &Position) -> Ordering {
        other
            .time
            .cmp(&self.time)
            .then(other.pos.0.cmp(&self.pos.0))
            .then(other.pos.1.cmp(&self.pos.1))
    }
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Position) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[test]
fn test_order() {
    let mut heap = BinaryHeap::new();
    heap.push(Position {
        time: 0,
        pos: (0, 0),
        gear: Gear::Torch,
    });
    heap.push(Position {
        time: 1,
        pos: (0, 0),
        gear: Gear::Torch,
    });
    heap.push(Position {
        time: 2,
        pos: (0, 0),
        gear: Gear::Torch,
    });

    let a = heap.pop().unwrap();
    let b = heap.pop().unwrap();
    let c = heap.pop().unwrap();
    assert_eq!(a.time, 0);
    assert_eq!(b.time, 1);
    assert_eq!(c.time, 2);
}

impl Maze {
    fn new(depth: i32, target: (i32, i32)) -> Maze {
        Maze {
            tiles: HashMap::new(),
            entry: (0, 0),
            target,
            depth,
        }
    }

    fn geological_index(&mut self, c: (i32, i32)) -> i32 {
        if let Some(index) = self.tiles.get(&c) {
            *index
        } else {
            let index = if c == self.entry || c == self.target {
                0
            } else if c.1 == 0 {
                c.0 * 16807
            } else if c.0 == 0 {
                c.1 * 48271
            } else {
                self.erosion_level((c.0 - 1, c.1)) * self.erosion_level((c.0, c.1 - 1))
            };
            self.tiles.insert(c, index);
            index
        }
    }

    fn erosion_level(&mut self, c: (i32, i32)) -> i32 {
        (self.geological_index(c) + self.depth) % 20183
    }

    fn get_terrain(&mut self, c: (i32, i32)) -> Terrain {
        let erosion = self.erosion_level(c);
        match erosion % 3 {
            0 => Terrain::Rocky,
            1 => Terrain::Wet,
            2 => Terrain::Narrow,
            _ => panic!("Weird terrain"),
        }
    }

    fn get_risk_level(&mut self, c: (i32, i32)) -> i32 {
        match self.get_terrain(c) {
            Terrain::Rocky => 0,
            Terrain::Wet => 1,
            Terrain::Narrow => 2,
        }
    }

    fn _print_terrain(&mut self, size: i32) {
        for y in 0..=size {
            for x in 0..=size {
                let c = (x, y);
                if c == self.entry {
                    print!("M");
                } else if c == self.target {
                    print!("T");
                } else {
                    let terrain = self.get_terrain(c);
                    print!(
                        "{}",
                        match terrain {
                            Terrain::Rocky => '.',
                            Terrain::Wet => '=',
                            Terrain::Narrow => '|',
                        }
                    );
                }
            }
            println!();
        }
    }

    fn area_risk(&mut self) -> i32 {
        let mut accum_risk = 0;
        for y in 0..=self.target.1 {
            for x in 0..=self.target.0 {
                accum_risk += self.get_risk_level((x, y));
            }
        }
        accum_risk
    }
}

fn solve_p1(depth: i32, target: (i32, i32)) -> i32 {
    let mut maze = Maze::new(depth, target);
    maze.area_risk()
}

fn neighbors_of(p: (i32, i32)) -> Vec<(i32, i32)> {
    let mut neighbors = Vec::new();
    if p.0 > 0 {
        neighbors.push((p.0 - 1, p.1));
    }
    if p.1 > 0 {
        neighbors.push((p.0, p.1 - 1));
    }
    neighbors.push((p.0 + 1, p.1));
    neighbors.push((p.0, p.1 + 1));
    neighbors
}

fn allowed_gear(terrain: Terrain, gear: Gear) -> bool {
    match terrain {
        Terrain::Rocky => gear != Gear::Neither,
        Terrain::Wet => gear != Gear::Torch,
        Terrain::Narrow => gear != Gear::Climbing,
    }
}

fn change_gear(terrain: Terrain, gear: Gear) -> Gear {
    match terrain {
        Terrain::Rocky => match gear {
            Gear::Climbing => Gear::Torch,
            Gear::Torch => Gear::Climbing,
            _ => panic!(),
        },
        Terrain::Wet => match gear {
            Gear::Climbing => Gear::Neither,
            Gear::Neither => Gear::Climbing,
            _ => panic!(),
        },
        Terrain::Narrow => match gear {
            Gear::Torch => Gear::Neither,
            Gear::Neither => Gear::Torch,
            _ => panic!(),
        },
    }
}

fn solve_p2(depth: i32, target: (i32, i32)) -> i32 {
    let mut maze = Maze::new(depth, target);
    let mut heap: BinaryHeap<Position> = BinaryHeap::new();
    let mut visited: HashMap<((i32, i32), Gear), i32> = HashMap::new();

    heap.push(Position {
        time: 0,
        pos: (0, 0),
        gear: Gear::Torch,
    });
    visited.insert(((0, 0), Gear::Torch), 0);
    while let Some(next_pos) = heap.pop() {
        let terrain = maze.get_terrain(next_pos.pos);
        let alt_eq = change_gear(terrain, next_pos.gear);

        let mut enqueu_visits = |time: i32, pos: (i32, i32), gear: Gear| {
            for n in neighbors_of(pos) {
                let terrain = maze.get_terrain(n);
                if !allowed_gear(terrain, gear) {
                    continue;
                }
                if let Some(t) = visited.get_mut(&(n, gear)) {
                    if *t <= time {
                        continue;
                    } else {
                        *t = time;
                    }
                } else {
                    visited.insert((n, gear), time);
                }

                heap.push(Position { time, pos: n, gear });
            }
        };

        if next_pos.pos == target {
            return next_pos.time + if next_pos.gear != Gear::Torch { 7 } else { 0 };
        }
        enqueu_visits(next_pos.time + 1, next_pos.pos, next_pos.gear);
        enqueu_visits(next_pos.time + 1 + 7, next_pos.pos, alt_eq);
    }
    panic!("Ran out of options...");
}

fn main() {
    println!("{}", solve_p1(11817, (9, 751)));
    println!("{}", solve_p2(11817, (9, 751)));
}

#[test]
fn test_p1() {
    assert_eq!(solve_p1(510, (10, 10)), 114);
}

#[test]
fn test_p2() {
    assert_eq!(solve_p2(510, (10, 10)), 45);
}
