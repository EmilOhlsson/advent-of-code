use std::collections::HashSet;

pub enum Turn {
    Left(isize),
    Right(isize),
}

enum Heading {
    North,
    West,
    South,
    East,
}

pub struct Path {
    x: isize,
    y: isize,
    heading: Heading,
    visited: HashSet<(isize, isize)>,
}

impl Path {
    pub fn walk(&mut self, turn: Turn) {
        let mut steps: isize;
        self.heading = match turn {
            Turn::Left(s) => {
                steps = s;
                match self.heading {
                    Heading::North => Heading::West,
                    Heading::West => Heading::South,
                    Heading::South => Heading::East,
                    Heading::East => Heading::North,
                }
            },

            Turn::Right(s) => {
                steps = s;
                match self.heading {
                    Heading::North => Heading::East,
                    Heading::East => Heading::South,
                    Heading::South => Heading::West,
                    Heading::West => Heading::North,
                }
            }
        };

        let dir = match self.heading {
            Heading::North => (0, 1),
            Heading::East => (1, 0),
            Heading::South => (0, -1),
            Heading::West => (-1, 0),
        };

        for _ in 0..steps {
            let new_coord = (self.x + dir.0, self.y + dir.1);
            if self.visited.contains(&new_coord) {
                println!("{:?}", new_coord);
            }
            self.x = new_coord.0;
            self.y = new_coord.1;
            self.visited.insert(new_coord);
        }
    }

    pub fn taxi_distance(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }

    pub fn new() -> Path {
        let mut path = Path {
            x: 0,
            y: 0,
            heading: Heading::North,
            visited: HashSet::new(),
        };
        path.visited.insert((0,0));
        path
    }
}

#[test]
fn test_walk1() {
    let mut path = Path::new();
    for step in vec![Turn::Right(2), Turn::Left(3)] {
        path.walk(step);
    }

    assert_eq!(path.taxi_distance(), 5);
}

#[test]
fn test_walk2() {
    let mut path = Path::new();
    for step in vec![Turn::Right(2), Turn::Right(2), Turn::Right(2)] {
        path.walk(step);
    }

    assert_eq!(path.taxi_distance(), 2);
}

#[test]
fn test_walk3() {
    let mut path = Path::new();
    for step in vec![Turn::Right(5), Turn::Left(5), Turn::Right(5), Turn::Right(3)] {
        path.walk(step);
    }

    assert_eq!(path.taxi_distance(), 12);
}
