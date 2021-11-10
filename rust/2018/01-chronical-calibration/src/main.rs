use std::collections::HashSet;
use std::collections::VecDeque;

fn solve_p1(input: &str) -> i32 {
    input.lines().map(|t| t.parse::<i32>().unwrap()).sum()
}

fn solve_p2(input: &str) -> i32 {
    let mods = input
        .split_whitespace()
        .map(|t| t.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();
    let mut seen: HashSet<i32> = HashSet::new();
    let mut back = 0;

    loop {
        let mut freqs = VecDeque::new();
        freqs = mods.iter().fold(freqs, |mut fs, m| {
            let back = *fs.back().unwrap_or(&back);
            fs.push_back(back + m);
            fs
        });
        back = *freqs.back().unwrap();

        for f in freqs.iter() {
            if seen.contains(f) {
                return *f;
            }
            seen.insert(*f);
        }
    }
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve_p1(input));
    println!("{}", solve_p2(input));
}
