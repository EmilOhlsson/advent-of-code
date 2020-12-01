use itertools::Itertools;

fn solve(input: &str, n: usize) -> u32 {
    let values = input
        .lines()
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .collect::<Vec<u32>>();
    for comb in values.iter().combinations(n) {
        if comb.iter().cloned().sum::<u32>() == 2020u32 {
            return comb.iter().cloned().product::<u32>();
        }
    }
    panic!("Did not find pair")
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve(input, 2));
    println!("{}", solve(input, 3));
}
