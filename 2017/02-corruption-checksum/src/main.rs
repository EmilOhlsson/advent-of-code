use std::fs::File;
use std::cmp;
use std::io::{BufRead, BufReader};

fn checksum(line: String) -> usize {
    let mm = line.split_whitespace()
        .map(str::parse::<usize>).map(Result::unwrap)
        .fold((std::usize::MAX, 0), |a, x| (cmp::min(x, a.0), cmp::max(x, a.1)));
    return mm.1 - mm.0;
}

fn main() {
    let reader = match File::open("input") {
        Ok(file) => BufReader::new(file),
        Err(e) => panic!("Unable to open file input.txt: {}", e),
    };

    let csum: usize = reader.lines().map(Result::unwrap).map(checksum).sum();
    println!("checksum: {}", csum);
}
