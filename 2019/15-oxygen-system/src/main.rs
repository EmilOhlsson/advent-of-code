pub mod intmachine;

use intmachine::{IOState, Intmachine};
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Clone, Copy, Debug)]
enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

enum Turn {
    Left,
    Right,
}

impl From<i64> for Direction {
    fn from(v: i64) -> Self {
        use Direction::*;
        match v {
            1 => North,
            2 => South,
            3 => West,
            4 => East,
            _ => panic!(),
        }
    }
}

impl Direction {
    fn turn(self, dir: Turn) -> Direction {
        use Direction::*;
        match dir {
            Turn::Left => match self {
                North => West,
                West => South,
                South => East,
                East => North,
            },
            Turn::Right => match self {
                North => East,
                East => South,
                South => West,
                West => North,
            },
        }
    }
}

#[derive(Debug, PartialEq)]
enum Tile {
    Wall,
    Open,
    OxygenTank,
}

#[derive(Debug)]
enum Status {
    Blocked = 0,
    Moved = 1,
    MovedToTank = 2,
}

impl From<i64> for Status {
    fn from(v: i64) -> Self {
        match v {
            0 => Status::Blocked,
            1 => Status::Moved,
            2 => Status::MovedToTank,
            other => panic!("{} is not a valid status", other),
        }
    }
}

#[derive(Debug)]
struct BotRep {
    pos: (i32, i32),
    dir: Direction,
}

impl BotRep {
    fn new() -> BotRep {
        BotRep {
            pos: (0, 0),
            dir: Direction::North,
        }
    }
    fn next(&self) -> (i32, i32) {
        match self.dir {
            Direction::North => (self.pos.0, self.pos.1 - 1),
            Direction::West => (self.pos.0 - 1, self.pos.1),
            Direction::South => (self.pos.0, self.pos.1 + 1),
            Direction::East => (self.pos.0 + 1, self.pos.1),
        }
    }
    fn step(&mut self) -> (i32, i32) {
        self.pos = self.next();
        self.pos
    }
    fn turn(&mut self, turn: Turn) {
        self.dir = self.dir.turn(turn)
    }
}

fn _print_labyrinth(labyrinth: &HashMap<(i32, i32), Tile>, botpos: (i32, i32)) {
    use itertools::Itertools;
    let xs = labyrinth
        .keys()
        .map(|k| k.0)
        .minmax()
        .into_option()
        .unwrap();
    let ys = labyrinth
        .keys()
        .map(|k| k.1)
        .minmax()
        .into_option()
        .unwrap();
    for y in ys.0..=ys.1 {
        for x in xs.0..=xs.1 {
            if (x, y) == botpos {
                print!("D");
            } else if (x, y) == (0, 0) {
                print!("S");
            }
            else {
                print!(
                    "{}",
                    labyrinth
                        .get(&(x, y))
                        .map(|t| match t {
                            Tile::Open => '.',
                            Tile::Wall => '#',
                            Tile::OxygenTank => 'O',
                        })
                        .unwrap_or('?')
                );
            }
        }
        println!();
    }
}

fn get_around(
    visited: &HashSet<(i32, i32)>,
    labyrinth: &HashMap<(i32, i32), Tile>,
    (x, y): (i32, i32),
) -> Vec<(i32, i32)> {
    [(x - 1, y), (x, y - 1), (x + 1, y), (x, y + 1)]
        .iter()
        .filter(|p| !visited.contains(p) && labyrinth.get(p).unwrap() != &Tile::Wall)
        .cloned()
        .collect::<Vec<(i32, i32)>>()
}

// Do a BFS visit of labyrinth
fn find_shortest_path(labyrinth: &HashMap<(i32, i32), Tile>) -> u32 {
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut queue = VecDeque::<(i32, i32)>::new();
    let mut came_from = HashMap::<(i32, i32), (i32, i32)>::new();

    queue.push_back((0, 0));

    while !queue.is_empty() {
        let pos = queue.pop_front().unwrap();
        if visited.insert(pos) {
            if labyrinth.get(&pos).unwrap() == &Tile::OxygenTank {
                let mut dist = 0;
                let mut curr = pos;
                while let Some(prev) = came_from.get(&curr) {
                    dist += 1;
                    curr = *prev;
                }
                return dist;
            }
            for next in get_around(&visited, &labyrinth, pos) {
                came_from.insert(next, pos);
                queue.push_back(next);
            }
        }
    }

    panic!("Was not able to find path");
}

fn fill_oxygen(labyrinth: &HashMap<(i32, i32), Tile>, tank_pos: (i32, i32)) -> u32 {
    let mut visited = HashSet::<(i32, i32)>::new();
    let mut queue = VecDeque::<(i32, i32)>::new();
    let mut distance = HashMap::<(i32, i32), u32>::new();

    queue.push_back(tank_pos);

    while !queue.is_empty() {
        let pos = queue.pop_front().unwrap();
        if visited.insert(pos) {
            for next in get_around(&visited, &labyrinth, pos) {
                distance.insert(next, distance.get(&pos).unwrap_or(&0) + 1);
                queue.push_back(next);
            }
        }
    }
    *distance.values().max().unwrap()
}

fn solve(input: &str) -> (u32, u32) {
    let mut labyrinth = HashMap::<(i32, i32), Tile>::new();
    let mut botrep = BotRep::new();
    let mut droid = Intmachine::load(input);
    let mut found = false;
    let mut tank = None;

    // We start in an open tile
    labyrinth.insert(botrep.pos, Tile::Open);

    // Follow wall, until back where we started
    loop {
        if droid.run_to_event(Some(botrep.dir as i64)) != IOState::Input {
            panic!("Expected to have consumed input")
        }
        if let IOState::Output(out) = droid.run_to_event(None).map(Status::from) {
            match out {
                Status::Blocked => {
                    labyrinth.insert(botrep.next(), Tile::Wall);
                    botrep.turn(Turn::Left);
                }
                Status::Moved => {
                    labyrinth.insert(botrep.step(), Tile::Open);
                    if botrep.pos == (0, 0) && found {
                        break;
                    }
                    botrep.turn(Turn::Right);
                }
                Status::MovedToTank => {
                    labyrinth.insert(botrep.step(), Tile::OxygenTank);
                    tank = Some(botrep.pos);
                    found = true;
                }
            }
        } else {
            panic!("Expected output")
        }
        //_print_labyrinth(&labyrinth, botrep.pos);
        //std::thread::sleep(std::time::Duration::from_millis(100));
    }

    (
        find_shortest_path(&labyrinth),
        fill_oxygen(&labyrinth, tank.unwrap()),
    )
}

fn main() {
    let input = include_str!("input");
    println!("{:?}", solve(input));
}
