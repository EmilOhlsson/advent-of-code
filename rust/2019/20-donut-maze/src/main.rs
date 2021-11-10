use std::collections::{HashMap, HashSet, VecDeque};

type Pos = (i32, i32);

#[derive(Debug)]
enum Tile {
    Open,
    Wall,
    InnerPortal(Pos),
    OuterPortal(Pos),
}

#[derive(Debug)]
struct Maze {
    start: Pos,
    goal: Pos,
    maze: HashMap<Pos, Tile>,
}

impl Maze {
    fn build(input: &str) -> Maze {
        let mut start: Option<Pos> = None;
        let mut goal: Option<Pos> = None;
        let mut half_portals = HashMap::<(char, char), Pos>::new();
        let mut maze = HashMap::new();

        let ir = input
            .lines()
            .map(|line| line.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();
        for r in 0..ir.len() {
            for c in 0..ir[r].len() {
                if ir[r][c].is_alphabetic() {
                    // if r || c == 0 || len, then outer, otherwise inner
                    // Create portal identifier, and point. Point should be next to an open space
                    let portal: Option<((char, char), Pos)> = if ir[r + 1][c].is_alphabetic() {
                        let key = (ir[r][c], ir[r + 1][c]);
                        if ir[r + 2][c] == '.' {
                            Some((key, (r as i32 + 2, c as i32)))
                        } else {
                            Some((key, (r as i32 - 1, c as i32)))
                        }
                    } else if ir[r][c + 1].is_alphabetic() {
                        let key = (ir[r][c], ir[r][c + 1]);
                        if ir[r][c + 2] == '.' {
                            Some((key, (r as i32, c as i32 + 2)))
                        } else {
                            Some((key, (r as i32, c as i32 - 1)))
                        }
                    } else {
                        None
                    };
                    if let Some((key, point)) = portal {
                        if key == ('A', 'A') {
                            start = Some(point);
                            maze.insert(point, Tile::Open);
                        } else if key == ('Z', 'Z') {
                            goal = Some(point);
                            maze.insert(point, Tile::Open);
                        } else if let Some(other) = half_portals.insert(key, point) {
                            let portal_a = if point.0 == 3
                                || point.0 == ir.len() as i32 - 4
                                || point.1 == 3
                                || point.1 == ir[r].len() as i32 - 4
                            {
                                Tile::OuterPortal(other)
                            } else {
                                Tile::InnerPortal(other)
                            };
                            let portal_b = if other.0 == 3
                                || other.0 == ir.len() as i32 - 4
                                || other.1 == 3
                                || other.1 == ir[r].len() as i32 - 4
                            {
                                Tile::OuterPortal(point)
                            } else {
                                Tile::InnerPortal(point)
                            };
                            maze.insert(point, portal_a);
                            maze.insert(other, portal_b);
                        }
                    }
                } else {
                    let p = (r as i32, c as i32);
                    if ir[r][c] == '.' && maze.get(&p).is_none() {
                        maze.insert((r as i32, c as i32), Tile::Open);
                    } else if ir[r][c] == '#' && maze.get(&p).is_none() {
                        maze.insert((r as i32, c as i32), Tile::Wall);
                    }
                }
            }
        }
        Maze {
            start: start.unwrap_or((0, 0)),
            goal: goal.unwrap_or((0, 0)),
            maze,
        }
    }

    fn _print_maze(&self) {
        let r_min = *self.maze.keys().map(|(r, _)| r).min().unwrap();
        let r_max = *self.maze.keys().map(|(r, _)| r).max().unwrap();
        let c_min = *self.maze.keys().map(|(_, c)| c).min().unwrap();
        let c_max = *self.maze.keys().map(|(_, c)| c).max().unwrap();

        for r in r_min..=r_max {
            for c in c_min..=c_max {
                if let Some(tile) = self.maze.get(&(r, c)) {
                    if (r, c) == self.start {
                        print!("S");
                    } else if (r, c) == self.goal {
                        print!("G");
                    } else {
                        match tile {
                            Tile::Open => print!("."),
                            Tile::Wall => print!("#"),
                            Tile::InnerPortal(_) => print!("I"),
                            Tile::OuterPortal(_) => print!("O"),
                        }
                    }
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }

    fn adjacent(&self, (x, y): Pos) -> Vec<(Pos, u32)> {
        let mut neighbors = Vec::new();
        for (dx, dy) in &[(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let p = (x + dx, y + dy);
            if let Some(p_t) = self.maze.get(&p) {
                match p_t {
                    Tile::Open => neighbors.push((p, 1)),
                    Tile::InnerPortal(p_n) => neighbors.push((*p_n, 2)),
                    Tile::OuterPortal(p_n) => neighbors.push((*p_n, 2)),
                    Tile::Wall => (),
                }
            }
        }
        neighbors
    }

    fn adjacent_v2(&self, (x, y): Pos, z: i32) -> Vec<(Pos, u32, i32)> {
        let mut neighbors = Vec::new();
        for (dx, dy) in &[(-1, 0), (0, -1), (1, 0), (0, 1)] {
            let p = (x + dx, y + dy);
            if let Some(p_t) = self.maze.get(&p) {
                match p_t {
                    Tile::Open => neighbors.push((p, 1, z)),
                    Tile::InnerPortal(p_n) => neighbors.push((*p_n, 2, z + 1)),
                    Tile::OuterPortal(p_n) => {
                        if z > 0 {
                            neighbors.push((*p_n, 2, z - 1))
                        } else {
                        }
                    }
                    Tile::Wall => (),
                }
            }
        }
        neighbors
    }

    fn bfs(&self) -> u32 {
        let mut visited = HashSet::<Pos>::new();
        let mut queue = VecDeque::<(Pos, u32)>::new();

        queue.push_back((self.start, 0));
        while let Some((p, d)) = queue.pop_front() {
            if visited.insert(p) {
                if p == self.goal {
                    return d;
                }
                for (p_n, d_n) in self.adjacent(p) {
                    queue.push_back((p_n, d + d_n));
                }
            }
        }
        panic!("Didn't find solution");
    }

    fn bfs_v2(&self) -> u32 {
        let mut visited = HashSet::<(Pos, i32)>::new();
        let mut queue = VecDeque::<(Pos, u32, i32)>::new();

        queue.push_back((self.start, 0, 0));
        while let Some((p, d, z)) = queue.pop_front() {
            if visited.insert((p, z)) {
                if p == self.goal && z == 0 {
                    return d;
                }
                for (p_n, d_n, z_n) in self.adjacent_v2(p, z) {
                    queue.push_back((p_n, d + d_n, z_n));
                }
            }
        }
        panic!("Didn't find solution");
    }
}

fn solve(input: &str) -> u32 {
    let maze = Maze::build(input);
    maze.bfs()
}

fn solve_v2(input: &str) -> u32 {
    let maze = Maze::build(input);
    maze.bfs_v2()
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input));
    println!("{}", solve_v2(input));
}

#[test]
fn test_simple_p1() {
    assert_eq!(solve(include_str!("input-simple")), 23);
}

#[test]
fn test_simple_p2() {
    assert_eq!(solve_v2(include_str!("input-simple")), 26);
}

#[test]
fn test_p2() {
    assert_eq!(solve_v2(include_str!("input-test-p2")), 396);
}

#[test]
fn test() {
    assert_eq!(solve(include_str!("input-test")), 58);
}
