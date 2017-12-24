use std::collections::HashMap;

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
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
}

struct Virus {
    position: (isize, isize),
    direction: Direction,
    board: HashMap<(isize, isize), bool>,
}

impl Virus {
    fn new(infects: &[Vec<bool>]) -> Virus {
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
            let direction_new;
            let position = self.position;
            let infected = self.board.entry(position).or_insert(false);
            if *infected {
                direction_new = self.direction.right();
            } else {
                direction_new = self.direction.left();
            }
            *infected = !*infected;
            result = *infected;
            direction_new
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
    let mut infects: Vec<Vec<bool>> = Vec::new();

    for line in input.lines() {
        let mut f: Vec<bool> = Vec::new();
        for ch in line.trim().chars() {
            f.push(ch == '#');
        }
        infects.push(f);
    }

    let mut count = 0;
    let mut virus = Virus::new(&infects);
    for _ in 0..10_000 {
        if virus.step() {
            count += 1;
        };
    }

    println!("{}", count);
}

#[test]
fn test_simple() {
    let input = include_str!("input-simple");
    let mut infects: Vec<Vec<bool>> = Vec::new();

    for line in input.lines() {
        let mut f: Vec<bool> = Vec::new();
        for ch in line.trim().chars() {
            f.push(if ch == '#' { true } else { false });
        }
        infects.push(f);
    }

    let mut count = 0;
    let mut virus = Virus::new(&infects);
    for _ in 0..10_000 {
        if virus.step() {
            count += 1;
        };
    }

    assert_eq!(count, 5587);
}
