use std::fs::File;
use std::io::{BufReader,BufRead};
use std::collections::HashMap;

fn main() {
    let mut word: [HashMap<char, usize>; 8] = [
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
        HashMap::new(),
    ];
    let reader = match File::open("input.txt") {
        Ok(file) => BufReader::new(file),
        Err(e) => panic!("Unable to open file: {}", e),
    };

    for line in reader.lines() {
        for (i, ch) in line.unwrap().chars().enumerate() {
            *word[i].entry(ch).or_insert(0) += 1;
        }
    }
    for ch_map in word.iter() {
        let mut ch_freq: Vec<(&char, &usize)> = ch_map.iter().collect();
        ch_freq.sort_by(|a, b| b.1.cmp(a.1));
        print!("{}", ch_freq[ch_freq.len() - 1].0);
    }
    println!("");
}
