pub mod intmachine;

use std::collections::HashMap;
use intmachine::{IOState, Intmachine};

#[derive(Debug, Clone, Copy)]
enum Dir {
    Up, Left, Down, Right
}

fn left(dir: Dir) -> Dir {
    use Dir::*;
    match dir {
        Up => Left,
        Left => Down,
        Down => Right,
        Right => Up,
    }
}

fn right(dir: Dir) -> Dir {
    use Dir::*;
    match dir {
        Up => Right,
        Right => Down,
        Down => Left,
        Left => Up,
    }
}

fn step(dir: Dir, pos: (i32, i32)) -> (i32, i32) {
    use Dir::*;
    match dir {
        Up => (pos.0, pos.1 + 1),
        Left => (pos.0 - 1, pos.1),
        Right => (pos.0 + 1, pos.1),
        Down => (pos.0, pos.1 - 1),
    }
}

fn solve(input: &str) -> usize {
    let mut machine = Intmachine::load(input);

    let mut dir = Dir::Up;
    let mut pos = (0, 0);
    let mut signal = 0;
    let mut color = true;
    let mut hull: HashMap<(i32, i32), i64> = HashMap::new();
    
    loop {
        match machine.run_to_event(signal) {
            IOState::Input => (),
            IOState::Done => {
                return hull.keys().count();
            }
            IOState::Output(output) => {
                if color {
                    hull.insert(pos, output);
                } else {
                    if output == 0 {
                        dir = left(dir);
                    } else {
                        dir = right(dir);
                    }
                    pos = step(dir, pos);
                    signal = *hull.get(&pos).unwrap_or(&0);
                }
                color = !color;
            }
        }
    }
}

fn print_hull(hull: &HashMap<(i32, i32), i64>) {
    let min_x = hull.iter().filter_map(|(p, c)| if *c != 0 { Some(p.0) } else { None }).min().unwrap();
    let max_x = hull.iter().filter_map(|(p, c)| if *c != 0 { Some(p.0) } else { None }).max().unwrap();
    let min_y = hull.iter().filter_map(|(p, c)| if *c != 0 { Some(p.1) } else { None }).min().unwrap();
    let max_y = hull.iter().filter_map(|(p, c)| if *c != 0 { Some(p.1) } else { None }).max().unwrap();

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            print!("{}", if hull.get(&(x, y)).unwrap() != &0 { '#' } else { ' ' });
        }
        println!();
    }
}

fn solve_p2(input: &str) {
    let mut machine = Intmachine::load(input);

    let mut dir = Dir::Up;
    let mut pos = (0, 0);
    let mut signal = 1;
    let mut color = true;
    let mut hull: HashMap<(i32, i32), i64> = HashMap::new();
    
    loop {
        match machine.run_to_event(signal) {
            IOState::Input => (),
            IOState::Done => {
                print_hull(&hull);
                return;
            }
            IOState::Output(output) => {
                if color {
                    hull.insert(pos, output);
                } else {
                    if output == 0 {
                        dir = left(dir);
                    } else {
                        dir = right(dir);
                    }
                    pos = step(dir, pos);
                    signal = *hull.get(&pos).unwrap_or(&0);
                }
                color = !color;
            }
        }
    }
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input));
    solve_p2(&input);
}
