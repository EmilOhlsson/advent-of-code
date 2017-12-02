use std::fs::File;
use std::io::{BufRead, BufReader};

fn checksum(line: String) -> usize {
    let vals = line.split_whitespace().map(str::parse::<usize>).map(Result::unwrap).collect::<Vec<_>>();
    for &n in &vals {
        if let Some(&o) = vals.iter().find(|&&o| o != n && o % n == 0) {
            return o / n;
        }
    }
    panic!("Could not find divisable on line");
}

fn main() {
    let reader = match File::open("input") {
        Ok(file) => BufReader::new(file),
        Err(e) => panic!("Unable to open file input.txt: {}", e),
    };

    let csum: usize = reader.lines().map(Result::unwrap).map(checksum).sum();
    println!("checksum: {}", csum);
}
