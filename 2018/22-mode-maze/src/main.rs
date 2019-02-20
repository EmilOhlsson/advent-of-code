use std::collections::HashMap;

enum Terrain {
    Rocky,
    Wet,
    Narrow,
}

struct Maze {
    tiles: HashMap<(u32, u32), u32>,
    entry: (u32, u32),
    target: (u32, u32),
    depth: u32,
}

impl Maze {
    fn new(depth: u32, target: (u32, u32)) -> Maze {
        Maze {
            tiles: HashMap::new(),
            entry: (0, 0),
            target,
            depth,
        }
    }

    fn geological_index(&mut self, c: (u32, u32)) -> u32 {
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

    fn erosion_level(&mut self, c: (u32, u32)) -> u32 {
        (self.geological_index(c) + self.depth) % 20183
    }

    fn get_terrain(&mut self, c: (u32, u32)) -> Terrain {
        let erosion = self.erosion_level(c);
        match erosion % 3 {
            0 => Terrain::Rocky,
            1 => Terrain::Wet,
            2 => Terrain::Narrow,
            _ => panic!("Weird terrain"),
        }
    }

    fn get_risk_level(&mut self, c: (u32, u32)) -> u32 {
        match self.get_terrain(c) {
            Terrain::Rocky => 0,
            Terrain::Wet => 1,
            Terrain::Narrow => 2,
        }
    }

    fn _print_terrain(&mut self, size: u32) {
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

    fn area_risk(&mut self) -> u32 {
        let mut accum_risk = 0;
        for y in 0..=self.target.1 {
            for x in 0..=self.target.0 {
                accum_risk += self.get_risk_level((x, y));
            }
        }
        accum_risk
    }
}

fn solve_p1(depth: u32, target: (u32, u32)) -> u32 {
    let mut maze = Maze::new(depth, target);
    maze.area_risk()
}

fn main() {
    println!("{}", solve_p1(11817, (9, 751)));
}

#[test]
fn test_p1() {
    assert_eq!(solve_p1(510, (10, 10)), 114);
}
