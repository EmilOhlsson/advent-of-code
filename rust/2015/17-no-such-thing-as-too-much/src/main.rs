fn bitmask(n: u32) -> u32 {
    (1 << n) - 1
}

fn bit_at(v: u32, pos: u32) -> u32 {
    (v >> pos) & 1
}

fn parse(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(str::trim)
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .collect()
}

fn solve(input: &str, liters: u32) -> u32 {
    let containers = parse(input);
    let len = containers.len() as u32;

    let mut count = 0;
    for mask in 0..=bitmask(len) {
        let space: u32 = (0..len)
            .map(|i| bit_at(mask, i) * containers[i as usize])
            .sum();
        if space == liters {
            count += 1
        }
    }

    count
}

fn solve_v2(input: &str, liters: u32) -> u32 {
    let containers = parse(input);
    let len = containers.len() as u32;

    let mut solutions = Vec::new();
    let mut minimum = bitmask(len);
    for mask in 0..=bitmask(len) {
        let space: u32 = (0..len)
            .map(|i| bit_at(mask, i) * containers[i as usize])
            .sum();
        if space == liters {
            solutions.push(mask);
            minimum = std::cmp::min(mask.count_ones(), minimum);
        }
    }
    solutions
        .iter()
        .filter(|v| v.count_ones() == minimum)
        .count() as u32
}
fn main() {
    let input = include_str!("input");
    println!("{}", solve(input, 150));
    println!("{}", solve_v2(input, 150));
}

#[test]
fn test() {
    let input = include_str!("input-simple");
    assert_eq!(solve(input, 25), 4);
}
