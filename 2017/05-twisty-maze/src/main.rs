use std::fs::File;
use std::io::{BufRead, BufReader};
fn main() {
    let reader = match File::open("input") {
        Ok(file) => BufReader::new(file),
        Err(e) => panic!("Unable to open file input.txt: {}", e),
    };

    let mut instructions = reader.lines()
        .map(Result::unwrap)
        .map(|s| s.as_str().parse::<isize>().unwrap())
        .collect::<Vec<isize>>();
    let mut jumps = 0;
    let mut i = 0;
    let mut i_p = 0;
    loop {
        i += instructions[i as usize];
        instructions[i_p as usize] += if instructions[i_p as usize] >= 3 { -1 } else { 1 };
        jumps += 1;
        if i >= instructions.len() as isize || i < 0 { break; }
        i_p = i;
    }
    println!("jumps: {}", jumps);
}
