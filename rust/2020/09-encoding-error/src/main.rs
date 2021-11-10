use itertools::Itertools;

fn read_input(input: &str) -> Vec<u64> {
    input
        .lines()
        .map(str::parse::<u64>)
        .map(Result::unwrap)
        .collect::<Vec<u64>>()
}

fn solve(preamble: usize, sequence: &[u64]) -> u64 {
    for (numbers, sum) in sequence
        .windows(preamble)
        .zip(sequence.iter().skip(preamble))
    {
        if !numbers
            .iter()
            .combinations(2)
            .any(|terms| terms.iter().cloned().sum::<u64>() == *sum)
        {
            return *sum;
        }
    }
    panic!("Did not find number");
}

fn solve_v2(number: u64, sequence: &[u64]) -> u64 {
    for window_size in 2.. {
        for window in sequence.windows(window_size) {
            if window.iter().sum::<u64>() == number {
                let minmax = window.iter().minmax().into_option().unwrap();
                return minmax.0 + minmax.1;
            }
        }
    }
    panic!("Did not find sequence");
}

fn main() {
    let input = include_str!("input");
    let sequence = read_input(input);
    let part1 = solve(25, &sequence);
    println!("{}", part1);
    println!("{}", solve_v2(part1, &sequence));
}

#[test]
fn test() {
    let input = include_str!("input-simple");
    let sequence = read_input(input);
    assert_eq!(solve(5, &sequence), 127);
    assert_eq!(solve_v2(127, &sequence), 62);
}
