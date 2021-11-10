use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::{HashSet,BTreeMap};

type Word = BTreeMap<char, usize>;

fn as_word(string: String) -> Word {
	let mut word = Word::new();
	string.chars().for_each(|c| {
		let entry = word.entry(c).or_insert(0);
		*entry += 1;
	});
	return word;
}

fn check_phrase(line: String) -> bool {
	let mut set = HashSet::new();
	let mut valid = true;
	line
		.split_whitespace()
		.for_each(|t| {
			let tok = String::from(t);
			let word = as_word(tok);
			if set.contains(&word) {valid = false;}
			else {set.insert(word);}
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
