use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut buffer = String::new();
    let mut file = File::open("startup.txt").unwrap();
    file.read_to_string(&mut buffer).unwrap();

    let code = buffer.chars().filter_map(|c| c.to_digit(10)).collect::<Vec<u32>>();
    let mut sum = 0;
    for (i, &n) in code.iter().enumerate() {
        let i_n = (i + 1) % code.len();
        sum += if n == code[i_n] { n } else { 0 };
    }
    println!("sum: {}", sum);
}
