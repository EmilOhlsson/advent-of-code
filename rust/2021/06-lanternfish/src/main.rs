use cached::proc_macro::cached;

#[cached]
fn fish(days: u64, state: u32) -> u64 {
    if days == 0 {
        1
    } else if state == 0 {
        fish(days - 1, 6) + fish(days - 1, 8)
    } else {
        fish(days - 1, state - 1)
    }
}

fn solve_v2(input: &str, days: u64) -> u64 {
    input
        .trim()
        .split(',')
        .map(str::parse::<u32>)
        .map(Result::unwrap)
        .map(|age| fish(days, age))
        .sum()
}

fn main() {
    let input = include_str!("input");
    println!("{}", solve_v2(input, 80));
    println!("{}", solve_v2(input, 256));
}

#[test]
fn test_simple() {
    let input = "3,4,3,1,2";
    assert_eq!(solve_v2(input, 18), 26);
    assert_eq!(solve_v2(input, 80), 5934);
    assert_eq!(solve_v2(input, 256), 26984457539);
}
