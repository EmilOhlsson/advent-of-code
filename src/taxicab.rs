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
    pub fn walk(&mut self, turn: Turn) -> bool {
        self.heading = match turn {
            Turn::Left(steps) => {
                match self.heading {
                    Heading::North => {
                        self.x -= steps;
                        Heading::West
                    },
                    Heading::West => {
                        self.y -= steps;
                        Heading::South
                    }
                    Heading::South => {
                        self.x += steps;
                        Heading::East
                    }
                    Heading::East => {
                        self.y += steps;
                        Heading::North
                    }
                }
            },

            Turn::Right(steps) => {
                match self.heading {
                    Heading::North => {
                        self.x += steps;
                        Heading::East
                    },
                    Heading::East => {
                        self.y -= steps;
                        Heading::South
                    }
                    Heading::South => {
                        self.x -= steps;
                        Heading::West
                    }
                    Heading::West => {
                        self.y += steps;
                        Heading::North
                    }
                }
            }
        };
        false
    }

    pub fn taxi_distance(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }

    pub fn new() -> Path {
        Path {
            x: 0,
            y: 0,
            heading: Heading::North,
            visited: HashSet::new(),
        }
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
