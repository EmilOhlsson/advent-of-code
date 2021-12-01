fn solve_p1(input: &str) -> u32 {
    let (_, incs) = input
        .lines()
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .fold((0, 0), |(prev, sum), depth| {
            (depth, sum + (depth > prev) as u32)
        });
    incs - 1
}

fn solve_p2(input: &str) -> u32 {
    let (_, incs) = input
        .lines()
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .collect::<Vec<u32>>()
        .windows(3)
        .fold((0, 0), |(prev, sum), depths| {
            let depth_sum = depths.iter().sum();
            (depth_sum, sum + (depth_sum > prev) as u32)
        });
    incs - 1
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve_p1(input));
    println!("{}", solve_p2(input));
}
