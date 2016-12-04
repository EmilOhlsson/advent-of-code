use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn verify_triangle(a: usize, b: usize, c: usize) -> bool {
    (a + b > c) && (c + b > a) && (a + c > b)
}

#[test]
fn impossible() {
    assert_eq!(verify_triangle(5, 10, 25), false);
}

#[test]
fn simple() {
    assert_eq!(verify_triangle(1, 1, 1), true);
}

fn main() {
    let reader = match File::open("input.txt") {
        Ok(file) => BufReader::new(file),
        Err(e) => panic!("Unable to open file: {}", e),
    };

    let mut valid_triangles: usize = 0;
    for l in reader.lines() {
        let line = l.unwrap();
        let toks: Vec<&str> = line.split_whitespace().collect();
        let a = toks[0].parse::<usize>().unwrap();
        let b = toks[1].parse::<usize>().unwrap();
        let c = toks[2].parse::<usize>().unwrap();

        valid_triangles += if verify_triangle(a, b, c) {1} else {0}
    }
    println!("Found {} valid triangles", valid_triangles);
}
