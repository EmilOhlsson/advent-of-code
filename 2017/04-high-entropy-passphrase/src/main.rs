use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet};

fn check_phrase(line: String) -> bool {
	let mut set = HashSet::new();
	let mut valid = true;
	line
		.split_whitespace()
		.for_each(|t| {
			let tok = String::from(t);
			if set.contains(&tok) {valid = false;}
			else {set.insert(tok);}
		});
	return valid;
}

fn main() {
    let reader = match File::open("input.txt") {
        Ok(file) => BufReader::new(file),
        Err(e) => panic!("Unable to open file input.txt: {}", e),
    };

    let nvalid: usize = reader.lines()
    	.map(Result::unwrap)
    	.map(|line| if check_phrase(line) {1} else {0})
    	.sum();
    println!("checksum: {}", nvalid);
}
