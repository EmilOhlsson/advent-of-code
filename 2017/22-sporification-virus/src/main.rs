use std::collections::HashMap;

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Inf {
    Clean,
    Weakend,
    Infected,
    Flagged,
}

impl Direction {
    fn left(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn right(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn back(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

struct Virus {
    position: (isize, isize),
    direction: Direction,
    board: HashMap<(isize, isize), Inf>,
}

impl Virus {
    fn new(infects: &[Vec<Inf>]) -> Virus {
        let mut board = HashMap::new();

        let len = infects.len();
        for (r, row) in infects.iter().enumerate() {
            for (c, &inf) in row.iter().enumerate() {
                board.insert((r as isize, c as isize), inf);
            }
        }

        Virus {
            position: (len as isize / 2, len as isize / 2),
            direction: Direction::Up,
            board: board,
        }
    }

    fn step(&mut self) -> bool {
        let result;
        self.direction = {
            let infected = self.board.entry(self.position).or_insert(Inf::Clean);
            let (new_inf, dir) = match *infected {
                Inf::Clean => (Inf::Weakend, self.direction.left()),
                Inf::Weakend => (Inf::Infected, self.direction),
                Inf::Infected => (Inf::Flagged, self.direction.right()),
                Inf::Flagged => (Inf::Clean, self.direction.back()),
            };
            result = new_inf == Inf::Infected;
            *infected = new_inf;
            dir
        };

        let (r, c) = self.position;
        let (dr, dc) = match self.direction {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        };
        self.position = (r + dr, c + dc);
        result
    }
}

fn main() {
    let input = include_str!("input");
    let mut infects: Vec<Vec<Inf>> = Vec::new();

    for line in input.lines() {
        let mut f: Vec<Inf> = Vec::new();
        for ch in line.trim().chars() {
            f.push(if ch == '#' { Inf::Infected } else { Inf::Clean });
        }
        infects.push(f);
    }

    let mut virus = Virus::new(&infects);
    let mut count = 0;
    for _ in 0..10_000_000 {
        if virus.step() {
            count += 1;
        }
    }

    println!("{}", count);
}

#[test]
fn test_simple() {
    let input = include_str!("input-simple");
    let mut infects: Vec<Vec<Inf>> = Vec::new();

    for line in input.lines() {
        let mut f: Vec<Inf> = Vec::new();
        for ch in line.trim().chars() {
            f.push(if ch == '#' { Inf::Infected } else { Inf::Clean });
        }
        infects.push(f);
    }

    let mut virus = Virus::new(&infects);
    let mut count = 0;
    for _ in 0..100 {
        if virus.step() {
            count += 1;
        }
    }

    assert_eq!(count, 26);

    for _ in 100..10_000_000 {
        if virus.step() {
            count += 1;
        }
    }
    assert_eq!(count, 2511944);
}
