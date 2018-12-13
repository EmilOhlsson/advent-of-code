use coding_challenge_utils::coord::Cartesian;
use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Rail {
    Track,
    TurnLeft,
    TurnRight,
    Intersection,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Turn {
    Left,
    Straight,
    Right,
}

fn solve(input: &str) -> (i32, i32) {
    let mut linewidth = 0;
    let mut trains: Vec<(Cartesian, Direction, Turn, bool)> = Vec::new();
    let mut rail: HashMap<Cartesian, Rail> = HashMap::new();
    for (y, l) in input.lines().enumerate() {
        linewidth = std::cmp::max(l.len(), linewidth);
        for (x, c) in l.chars().enumerate() {
            if !c.is_whitespace() {
                let cord = Cartesian::new(x as i32, y as i32);
                match &c {
                    '|' | '-' => {
                        rail.insert(cord, Rail::Track);
                    }
                    '\\' => {
                        rail.insert(cord, Rail::TurnLeft);
                    }
                    '/' => {
                        rail.insert(cord, Rail::TurnRight);
                    }
                    '+' => {
                        rail.insert(cord, Rail::Intersection);
                    }
                    '^' => {
                        rail.insert(cord, Rail::Track);
                        trains.push((cord, Direction::Up, Turn::Left, true));
                    }
                    '<' => {
                        rail.insert(cord, Rail::Track);
                        trains.push((cord, Direction::Left, Turn::Left, true));
                    }
                    'v' => {
                        rail.insert(cord, Rail::Track);
                        trains.push((cord, Direction::Down, Turn::Left, true));
                    }
                    '>' => {
                        rail.insert(cord, Rail::Track);
                        trains.push((cord, Direction::Right, Turn::Left, true));
                    }
                    _ => (),
                };
            }
        }
    }

    loop {
        for i in 0..trains.len() {
            let train = trains[i];
            if !train.3 {
                continue;
            }
            let pos = train.0;
            let dir = train.1;
            let mut turn = train.2;

            if trains.iter().filter(|(_, _, _, alive)| *alive).count() == 1 {
                return (pos.x, pos.y);
            }

            let new_pos = match dir {
                Direction::Up => &pos + &Cartesian::new(0, -1),
                Direction::Down => &pos + &Cartesian::new(0, 1),
                Direction::Right => &pos + &Cartesian::new(1, 0),
                Direction::Left => &pos + &Cartesian::new(-1, 0),
            };

            if trains.iter().filter(|(_, _, _, alive)| *alive).count() == 1 {
                return (new_pos.x, new_pos.y);
            }

            let r = rail.get(&new_pos).unwrap();
            let new_dir = match &r {
                Rail::Intersection => {
                    let old_turn = turn;
                    turn = match old_turn {
                        Turn::Left => Turn::Straight,
                        Turn::Straight => Turn::Right,
                        Turn::Right => Turn::Left,
                    };
                    match dir {
                        Direction::Up => match old_turn {
                            Turn::Left => Direction::Left,
                            Turn::Straight => Direction::Up,
                            Turn::Right => Direction::Right,
                        },
                        Direction::Left => match old_turn {
                            Turn::Left => Direction::Down,
                            Turn::Straight => Direction::Left,
                            Turn::Right => Direction::Up,
                        },
                        Direction::Right => match old_turn {
                            Turn::Left => Direction::Up,
                            Turn::Straight => Direction::Right,
                            Turn::Right => Direction::Down,
                        },
                        Direction::Down => match old_turn {
                            Turn::Left => Direction::Right,
                            Turn::Straight => Direction::Down,
                            Turn::Right => Direction::Left,
                        },
                    }
                }
                Rail::TurnLeft => match dir {
                    Direction::Up => Direction::Left,
                    Direction::Left => Direction::Up,
                    Direction::Right => Direction::Down,
                    Direction::Down => Direction::Right,
                },
                Rail::TurnRight => match dir {
                    Direction::Up => Direction::Right,
                    Direction::Right => Direction::Up,
                    Direction::Left => Direction::Down,
                    Direction::Down => Direction::Left,
                },
                Rail::Track => dir,
            };

            let mut crash = false;

            for t in trains.iter_mut().filter(|(_, _, _, alive)| *alive).filter(|(p, _, _, _)| p == &new_pos) {
                crash = true;
                t.3 = false;
                println!("Crash at {:?}", new_pos);
            }

            trains[i] = (new_pos, new_dir, turn, !crash);
        }

        trains.sort_unstable_by_key(|(p, _, _, _)| p.x + p.y * linewidth as i32);

        //for i in 0..trains.len() {
        //    for j in 0..trains.len() {
        //        if j != i {
        //            if trains[i].3 && trains[j].3 && trains[i].0 == trains[j].0 { 
        //                println!("Crash at: {:?}", trains[i].0);
        //                trains[i].3 = false;
        //                trains[j].3 = false;
        //            }
        //        }
        //    }
        //}
    }
}

fn main() {
    let input = include_str!("input.txt");
    println!("{:?}", solve(input));
}

#[test]
fn test() {
    assert_eq!(solve(include_str!("input-simple-p2.txt")), (6, 4));
}
