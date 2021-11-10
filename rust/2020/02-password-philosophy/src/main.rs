use regex::Regex;

fn solve_p1(input: &str) -> u32 {
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)$").unwrap();
    let mut valid = 0;
    for cap in input.lines().map(|l| re.captures(l).unwrap()) {
        let lo = cap[1].parse::<usize>().unwrap();
        let hi = cap[2].parse::<usize>().unwrap();
        let ch = cap[3].chars().next().unwrap();
        let password = &cap[4];
        let count = password.chars().filter(|c| c == &ch).count();
        if lo <= count && count <= hi {
            valid += 1;
        }
    }

    valid
}

fn solve_p2(input: &str) -> u32 {
    let re = Regex::new(r"(\d+)-(\d+) (\w): (\w+)$").unwrap();
    let mut valid = 0;
    for cap in input.lines().map(|l| re.captures(l).unwrap()) {
        let lo = cap[1].parse::<usize>().unwrap() - 1;
        let hi = cap[2].parse::<usize>().unwrap() - 1;
        let ch = cap[3].chars().next().unwrap();
        let password = cap[4].chars().collect::<Vec<char>>();
        if (password[lo] == ch && password[hi] != ch) || (password[lo] != ch && password[hi] == ch) {
            valid += 1;
        }
    }

    valid
}
fn main() {
    let input = include_str!("input");
    println!("{}", solve_p1(input));
    println!("{}", solve_p2(input));
}
