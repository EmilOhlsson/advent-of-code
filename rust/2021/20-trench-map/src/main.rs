use std::cmp::{max, min};
use std::collections::HashSet;

type Xy = (i32, i32);
type XySet = HashSet<Xy>;

fn parse(input: &str) -> (Vec<bool>, XySet) {
    let mut section_iter = input.split("\n\n");
    let algorithm_lines = section_iter.next().unwrap();
    let algorithm = algorithm_lines
        .lines()
        .map(|line| line.chars().map(|ch| ch == '#'))
        .flatten()
        .collect::<Vec<bool>>();

    let image_lines = section_iter.next().unwrap();
    let mut set = XySet::new();
    for (row, line) in image_lines.lines().enumerate() {
        for (col, ch) in line.chars().enumerate() {
            if ch == '#' {
                set.insert((row as i32, col as i32));
            }
        }
    }
    (algorithm, set)
}

/// Returns (row_low, col_low, row_high, col_high)
fn get_span(image: &XySet) -> (i32, i32, i32, i32) {
    image.iter().fold(
        (i32::MAX, i32::MAX, i32::MIN, i32::MIN),
        |(r_lo, c_lo, r_hi, c_hi), (r, c)| {
            (min(r_lo, *r), min(c_lo, *c), max(r_hi, *r), max(c_hi, *c))
        },
    )
}

fn _print_image(image: &XySet) {
    let (r_lo, c_lo, r_hi, c_hi) = get_span(image);
    for r in r_lo..=r_hi {
        for c in c_lo..=c_hi {
            print!("{}", if image.contains(&(r, c)) { '#' } else { '.' });
        }
        println!()
    }
    println!()
}

fn get_index_at(image: &XySet, (row, col): (i32, i32)) -> usize {
    let mut result = 0;
    for r in (row - 1)..=(row + 1) {
        for c in (col - 1)..=(col + 1) {
            result <<= 1;
            result |= image.contains(&(r, c)) as usize;
        }
    }

    result
}

fn tick(generation: u32, algorithm: &[bool], mut image: XySet) -> XySet {
    let mut image_new = XySet::new();
    let (r_lo, c_lo, r_hi, c_hi) = get_span(&image);

    // Fill in boundry, to simulate infinite board
    if algorithm[0] && !algorithm[255] && generation % 2 == 0 {
        for r in (r_lo - 2)..=(r_hi + 2) {
            image.insert((r, c_lo - 2));
            image.insert((r, c_lo - 1));
            image.insert((r, c_hi + 1));
            image.insert((r, c_hi + 2));
        }
        for c in (c_lo - 2)..=(c_hi + 2) {
            image.insert((r_lo - 2, c));
            image.insert((r_lo - 1, c));
            image.insert((r_hi + 1, c));
            image.insert((r_hi + 2, c));
        }
    }

    for row in (r_lo - 1)..=(r_hi + 1) {
        for col in (c_lo - 1)..=(c_hi + 1) {
            if algorithm[get_index_at(&image, (row, col))] {
                image_new.insert((row, col));
            }
        }
    }
    image_new
}

fn solve(input: &str) -> (usize, usize) {
    let (algorithm, mut image) = parse(input);

    image = tick(1, &algorithm, image);
    image = tick(2, &algorithm, image);
    let part1 = image.len();

    for generation in 3..=50 {
        image = tick(generation, &algorithm, image);
    }
    let part2 = image.len();

    (part1, part2)
}

fn main() {
    let input = include_str!("input");
    println!("{:?}", solve(input));
}

#[test]
fn test_simple() {
    let input = include_str!("input-simple");
    assert_eq!(solve(input), (35, 3351));
}
