mod taxicab;

use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut buffer = String::new();
    let mut path = taxicab::Path::new();
    match File::open("input.txt") {
        Ok(mut file) => match file.read_to_string(&mut buffer) {
            Ok(_) => (),
            _ => panic!("Unable to read from input.txt"),
        },
        _ => panic!("Unable to open input.txt"),
    };

    for tok in buffer.split(",") {
        let (dir, steps_str) = tok.trim().split_at(1);
        let steps = match steps_str.parse::<isize>() {
            Ok(nsteps) => nsteps,
            _ => panic!("Unable to parse: {}", steps_str),
        };
        path.walk(match dir {
            "L" => taxicab::Turn::Left(steps),
            "R" => taxicab::Turn::Right(steps),
            _ => panic!("{} is not a valid direction"),
        });
    }
    
    println!("taxi distance: {}", path.taxi_distance());
}
